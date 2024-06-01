use crate::agreement::Agreement;
use crate::lamport::Signature as Lsignature;

#[derive(Clone, Debug, candid::CandidType, Serialize, Deserialize)]
pub struct Signature {
    pub agrees_to: Box<Agreement>,

    pub value: Lsignature,
}
