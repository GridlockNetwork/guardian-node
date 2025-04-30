use anyhow::{ anyhow, bail, Result };
use curv::cryptographic_primitives::secret_sharing::feldman_vss::{
    ShamirSecretSharing,
    VerifiableSS,
};
use curv::elliptic::curves::{ Curve, Point, Scalar };
use curv::BigInt;
use itertools::Itertools;

use crate::recovery::Party;

pub struct LinearShareParts<C> where C: Curve {
    pub retained: Scalar<C>,
    pub for_peer_exchange: Vec<Scalar<C>>,
}

pub struct RecoveryCalculator<C> where C: Curve {
    pub recovery_index: usize,
    pub party: Party,
    pub threshold: usize,
    pub secret_share: Scalar<C>,
}

impl<C> RecoveryCalculator<C> where C: Curve {
    pub fn new(
        recovery_index: usize,
        party_index: usize,
        all_parties: Vec<usize>,
        threshold: usize,
        secret_share: Scalar<C>
    ) -> Self {
        Self {
            recovery_index,
            party: Party {
                party_index,
                all_parties,
            },
            threshold,
            secret_share,
        }
    }

    pub fn create_secret_sharing_of_lost_share(&self) -> LinearShareParts<C> {
        let li = Self::map_share_to_new_params_for_x(
            self.recovery_index,
            self.party.party_index,
            &self.party.all_parties
        );
        let lc = self.secret_share.clone() * li;

        Self::create_linear_shares_of_scalar(lc, self.threshold)
    }

    pub fn create_secret_sharing_of_zero_point(&self) -> LinearShareParts<C> {
        let sss_params = ShamirSecretSharing {
            threshold: self.threshold as u16,
            share_count: 5,
        };
        let all_parties = &*self.party.all_parties
            .iter()
            .map(|&i| i as u16)
            .collect_vec();
        let li = VerifiableSS::<C>::map_share_to_new_params(
            &sss_params,
            self.party.party_index as u16,
            all_parties
        );

        let lc = self.secret_share.clone() * li;

        Self::create_linear_shares_of_scalar(lc, self.threshold)
    }

    fn create_linear_shares_of_scalar(
        scalar: Scalar<C>,
        num_of_shares: usize
    ) -> LinearShareParts<C> {
        let mut rij: Vec<Scalar<C>> = Vec::new();
        for _ in 0..num_of_shares {
            rij.push(Scalar::<C>::random());
        }

        let rij_sum = rij.iter().sum();

        let rii = scalar - &rij_sum;

        LinearShareParts {
            retained: rii,
            for_peer_exchange: rij,
        }
    }

    pub fn sum_secret_shares(&self, rii: Scalar<C>, mixed_shares: Vec<Scalar<C>>) -> Scalar<C> {
        mixed_shares.iter().sum::<Scalar<_>>() + rii
    }

    pub fn validate_recovered_share(
        secret: &Scalar<C>,
        vss: &[VerifiableSS<C>],
        index: usize
    ) -> Result<()> {
        let mut vss_iter = vss.iter();
        let head = vss_iter
            .next()
            .unwrap()
            .get_point_commitment(index as u16);
        let tail = vss_iter;
        let point_commitment_sum = tail.fold(head.clone(), |acc, x| {
            acc + x.get_point_commitment(index as u16)
        });
        let public_point = Point::generator() * secret.clone();
        match public_point == point_commitment_sum {
            true => Ok(()),
            false => bail!("Recovered key share did not pass validation"),
        }
    }

    pub fn calculate_y_sum_from_vss_vec(vss_vec: &[VerifiableSS<C>]) -> Result<Point<C>>
        where C: Curve
    {
        let mut vss_iter = vss_vec.iter();

        let add_to_sum = |sum: Point<C>, x: &VerifiableSS<C>| -> Result<Point<C>> {
            let new_p = x.commitments.first().ok_or(anyhow!("VSS commitments empty"))?;
            Ok(sum + new_p.clone())
        };

        let head = vss_iter
            .next()
            .ok_or(anyhow!("VSS empty"))?
            .commitments.first()
            .ok_or(anyhow!("VSS commitments empty"))?;
        let mut tail = vss_iter;
        let y_sum_from_vss = tail.try_fold(head.clone(), add_to_sum)?;

        Ok(y_sum_from_vss)
    }

    pub fn calculate_y_sum_from_single_vss(vss: &VerifiableSS<C>) -> Result<Point<C>> {
        let y_sum_from_vss = vss.commitments.first().ok_or(anyhow!("VSS commitments empty"))?;
        Ok(y_sum_from_vss.clone())
    }

    pub fn map_share_to_new_params_for_x(x_index: usize, index: usize, s: &[usize]) -> Scalar<C> {
        let s_len = s.len();
        let mut all_indices = s.to_vec();
        all_indices.push(index);
        all_indices.push(x_index);

        let max_index = all_indices.iter().max().unwrap();

        let points: Vec<Scalar<C>> = (0..=*max_index)
            .map(|i| {
                let index_bn = BigInt::from((i + 1) as u32);
                Scalar::from(&index_bn)
            })
            .collect::<Vec<Scalar<C>>>();

        let x = &points[x_index];
        let xi = &points[index];
        let num = Scalar::from(&BigInt::from(1u32));
        let denum = Scalar::from(&BigInt::from(1u32));
        let num = (0..s_len).fold(num, |acc, i| {
            if s[i] != index {
                let xj_sub_x = points[s[i]].clone() - x;
                acc * xj_sub_x
            } else {
                acc
            }
        });
        let denum = (0..s_len).fold(denum, |acc, i| {
            if s[i] != index {
                let xj_sub_xi = points[s[i]].clone() - xi;
                acc * xj_sub_xi
            } else {
                acc
            }
        });
        let denum = denum.invert().expect("Denum is not zero");
        num * denum
    }
}

#[derive(Clone, Debug)]
pub struct KeyshareRecoverySession {
    pub session_id: String,
    pub key_ids: Vec<String>,
    pub public_key: Option<String>,
}
