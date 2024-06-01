use crate::signature::Signature;
use crate::user::User;
use chrono::prelude::*;

#[derive(Clone, Debug)]
pub struct Agreement {
    pub terms: Vec<String>,
    pub by_user: User,
    pub with_user: User,
    pub date: DateTime<Utc>,
    pub proof_of_agreement: Option<ProofOfAgreement>,
    pub id: u64,
}
pub type ProofOfAgreement = (Option<Signature>, Option<Signature>);
