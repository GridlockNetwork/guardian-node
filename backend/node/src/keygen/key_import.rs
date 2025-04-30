use crate::command::{ JsonCommand, MsgContext };
use crate::storage::{ KeyshareSaver, SchnorrkelSecretKey, Sr25519 };
use anyhow::{ bail, Result };
use curv::cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS;
use curv::elliptic::curves::{ Ed25519, Scalar };
use serde::{ Deserialize, Serialize };
use std::convert::{ TryFrom, TryInto };

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyImportCommand {
    pub key_id: String,
    pub key_type: String,
    pub key: String,
    pub threshold: usize,
    pub share_count: usize,
}

impl JsonCommand for KeyImportCommand {
    type Response = Vec<KeyImportShareCommand>;

    fn execute_message(self, _ctx: MsgContext) -> Result<Self::Response> where Self: Sized {
        match self.key_type.as_str() {
            "sr25519" => bail!("sr25519 import not yet implemented"),
            "eddsa" => bail!("EdDSA import not yet implemented"),
            "ecdsa" => bail!("ECDSA import not yet implemented"),
            _ => bail!("Unknown type provided for key being imported"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct KeyImportShareCommand {
    pub key_id: String,
    pub key_type: String,
    pub key_share: String,
    pub vss: String,
    pub threshold: usize,
    pub index: usize,
    pub key: Option<String>,
}

impl TryFrom<KeyImportShareCommand> for Sr25519 {
    type Error = anyhow::Error;
    fn try_from(k: KeyImportShareCommand) -> Result<Self> {
        let secret = serde_json::from_str::<Scalar<Ed25519>>(&k.key_share)?;
        let vss = serde_json::from_str::<VerifiableSS<Ed25519>>(&k.vss)?;
        let secret_key = match k.key {
            None => None,
            Some(key) => Some(serde_json::from_str::<SchnorrkelSecretKey>(&key)?),
        };
        Ok(Self {
            secret_key,
            threshold: k.threshold,
            party_index: k.index,
            x_i: secret.into(),
            vss_scheme: vss.into(),
        })
    }
}

impl JsonCommand for KeyImportShareCommand {
    type Response = ();

    fn execute_message(self, _ctx: MsgContext) -> Result<Self::Response> where Self: Sized {
        match self.key_type.as_str() {
            "sr25519" => {
                let keyfile: Sr25519 = self.clone().try_into()?;
                let ks = KeyshareSaver::new_creator(&self.key_id);
                ks.save_key(&keyfile)
            }
            _ => bail!("Unknown type provided for key being imported"),
        }
    }
}
