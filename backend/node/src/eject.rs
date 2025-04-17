use anyhow::{ bail, Result };
use curv::elliptic::curves::{ Curve, Ed25519, Scalar, Secp256k1 };
use curv::{ cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS, BigInt };
use itertools::Itertools;
use serde::{ Deserialize, Serialize };
use tracing::{ error, info };

use crate::command::{ JsonCommand, MsgContext };
use crate::storage::{ KeyshareAccessor, ECDSA, EDDSA };

const THRESHOLD: usize = 3;

#[derive(Deserialize, Serialize, Debug)]
pub struct EjectInfo {
    pub key_id: String,
    pub share_info: EjectShareInfo,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct KeyReconstructionResult {
    pub key_id: String,
    pub key: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum EjectShareInfo {
    Secp256k1(Scalar<Secp256k1>, usize),
    Ed25519(Scalar<Ed25519>, usize),
}

impl From<EDDSA> for EjectShareInfo {
    fn from(ed: EDDSA) -> Self {
        Self::Ed25519(ed.x_i, ed.party_index)
    }
}

impl From<ECDSA> for EjectShareInfo {
    fn from(ec: ECDSA) -> Self {
        Self::Secp256k1(ec.x_i, ec.party_index)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EjectSharesCommand {
    key_ids_to_eject: Vec<String>,
}

impl JsonCommand for EjectSharesCommand {
    type Response = Vec<EjectInfo>;

    fn execute_message(self, _ctx: MsgContext) -> Result<Self::Response> where Self: Sized {
        let key_ids = self.key_ids_to_eject.into_iter().unique().collect::<Vec<String>>();

        retrieve_eject_info_from_key_ids(&key_ids)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EjectKeysCommand {
    key_ids: Vec<String>,
    eject_info: Vec<Vec<EjectInfo>>,
}

impl JsonCommand for EjectKeysCommand {
    type Response = Vec<KeyReconstructionResult>;

    fn execute_message(mut self, _ctx: MsgContext) -> Result<Self::Response> where Self: Sized {
        self.retrieve_keys()
    }
}

impl EjectKeysCommand {
    /// Combines two sets of imported keyshares with the set owned by this device to recover the associated private keys
    fn retrieve_keys(&mut self) -> Result<Vec<KeyReconstructionResult>> {
        let key_ids = self.key_ids.clone().into_iter().unique().collect::<Vec<String>>();
        let owned_shares = retrieve_eject_info_from_key_ids(&key_ids)?;

        self.eject_info.push(owned_shares);
        let reformed_keys = combine_keyshares(&key_ids, &self.eject_info);
        Ok(reformed_keys)
    }
}

fn retrieve_eject_info_from_key_ids(key_ids: &[String]) -> Result<Vec<EjectInfo>> {
    let eject_info = key_ids
        .iter()
        .filter_map(|key_id| {
            if let Ok(ka) = KeyshareAccessor::<ECDSA>::read_only(key_id) {
                let share_info = EjectShareInfo::from(ka.key);
                Some(EjectInfo {
                    key_id: key_id.to_string(),
                    share_info,
                })
            } else if let Ok(ka) = KeyshareAccessor::<EDDSA>::read_only(key_id) {
                let share_info = EjectShareInfo::from(ka.key);
                Some(EjectInfo {
                    key_id: key_id.to_string(),
                    share_info,
                })
            } else {
                None
            }
        })
        .collect();
    Ok(eject_info)
}

fn combine_keyshares(
    key_ids: &[String],
    eject_info_vec: &[Vec<EjectInfo>]
) -> Vec<KeyReconstructionResult> {
    key_ids
        .iter()
        .filter_map(|key_id| {
            let shares = collect_shares_by_key_id_from_supplied_keyshares(key_id, eject_info_vec);
            match reconstruct_key_from_collected_eject_info(&shares) {
                Ok(key) =>
                    Some(KeyReconstructionResult {
                        key_id: key_id.clone(),
                        key,
                    }),
                Err(err) => {
                    error!("Unable to reconstruct key with id {key_id}: {err}");
                    None
                }
            }
        })
        .collect()
}

fn reconstruct_key_from_collected_eject_info(eject_infos: &[EjectShareInfo]) -> Result<String> {
    if eject_infos.len() < THRESHOLD {
        bail!("Not enough keyshares found to reconstruct private key");
    }

    let mut secp_scalars = Vec::new();
    let mut ed25519_scalars = Vec::new();
    let mut indices = Vec::new();
    eject_infos.iter().for_each(|x| {
        match x {
            EjectShareInfo::Secp256k1(scalar, index) => {
                secp_scalars.push(scalar.clone());
                indices.push(*index);
            }
            EjectShareInfo::Ed25519(scalar, index) => {
                ed25519_scalars.push(scalar.clone());
                indices.push(*index);
            }
        }
    });

    let res = (if
        let Some(reconstructed_key) = reconstruct_key::<Secp256k1>(&indices, &secp_scalars)
    {
        serde_json::to_string(&reconstructed_key)
    } else if let Some(reconstructed_key) = reconstruct_key::<Ed25519>(&indices, &ed25519_scalars) {
        serde_json::to_string(&reconstructed_key)
    } else {
        bail!(
            "Not enough keyshares of same key type found to reconstruct private key (this shouldn't happen!)"
        );
    })?;
    Ok(res)
}

fn collect_shares_by_key_id_from_supplied_keyshares(
    key_id: &str,
    eject_info_vec: &[Vec<EjectInfo>]
) -> Vec<EjectShareInfo> {
    eject_info_vec
        .iter()
        .enumerate()
        .filter_map(|(set_index, eject_info_set)| {
            match eject_info_set.iter().find(|x| x.key_id == key_id) {
                Some(EjectInfo { key_id: _, share_info }) => Some(share_info.clone()),
                None => {
                    info!("No eject info for key id {} in set {}", key_id, set_index);
                    None
                }
            }
        })
        .collect::<Vec<EjectShareInfo>>()
}

fn reconstruct_key<C>(indices: &[usize], shares: &[Scalar<C>]) -> Option<Scalar<C>> where C: Curve {
    if shares.len() != indices.len() || shares.len() < THRESHOLD {
        return None;
    }

    let points = indices
        .iter()
        .map(|i| {
            let index_bn = BigInt::from(*i as u32);
            index_bn.into()
        })
        .collect::<Vec<Scalar<C>>>();
    Some(VerifiableSS::<C>::lagrange_interpolation_at_zero(&points, shares))
}

fn remove_first_and_last_and_backslash(value: &str) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().replace("\\", "")
}
