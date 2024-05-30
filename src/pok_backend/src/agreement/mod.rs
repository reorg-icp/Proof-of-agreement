use crate::signature::Signature;
use crate::user::User;
#[derive(Clone, Debug)]
pub struct Agreement {
    pub terms: Vec<String>,
    pub by_user: User,
    pub with_user: User,
    pub date: String,
    pub proof_of_agreement: Option<ProofOfAgreement>,
}
pub type ProofOfAgreement = (Option<Signature>, Option<Signature>);
