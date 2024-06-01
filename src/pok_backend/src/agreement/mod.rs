use std::borrow::Cow;

use crate::user::User;
use crate::{lamport::PublicKey, signature::Signature};
use candid::{Decode, Encode};
use chrono::prelude::*;
use ic_stable_structures::{BoundedStorable, Storable};

#[derive(Clone, Debug, candid::CandidType, Serialize, Deserialize)]
pub struct Agreement {
    pub terms: Vec<String>,
    pub by_user: User,
    pub with_user: User,
    pub date: String,
    pub proof_of_agreement: Option<ProofOfAgreement>,
    pub public_keys: Option<PublicKeys>,
    pub id: u64,
}

pub type ProofOfAgreement = (Option<Signature>, Option<Signature>);
pub type PublicKeys = (Option<PublicKey>, Option<PublicKey>);

impl Storable for Agreement {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Agreement {
    const MAX_SIZE: u32 = 200000;
    const IS_FIXED_SIZE: bool = false;
}
