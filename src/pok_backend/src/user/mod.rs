use crate::agreement::{self, Agreement, ProofOfAgreement};
use crate::signature::Signature;

use candid::Principal;
#[derive(Clone, Debug)]
pub struct User {
    pub identity: String,
}
pub trait CreateAgreement {
    fn new_agreement(self, terms: Vec<String>, date: String, with_user: User) -> Agreement;
}
pub trait Agree {
    fn agree(self, agreement: Agreement) -> Agreement;
}
impl CreateAgreement for User {
    fn new_agreement(self, terms: Vec<String>, date: String, with_user: User) -> Agreement {
        Agreement {
            by_user: self,
            with_user,
            terms,
            date,
            proof_of_agreement: None,
        }
    }
}

impl Agree for User {
    fn agree(self, mut agreement: Agreement) -> Agreement {
        let signature = Signature {
            agrees_to: Box::new(agreement.clone()),
            value: String::from("here is my signature"),
        };

        if let Some((first_sig_opt, second_sig_opt)) = &mut agreement.proof_of_agreement {
            match first_sig_opt {
                Some(_) => {
                    let new_agreement = (first_sig_opt.take(), Some(signature));
                    agreement.proof_of_agreement = Some(new_agreement);
                }
                None => {
                    let new_agreement = (Some(signature), second_sig_opt.take());
                    agreement.proof_of_agreement = Some(new_agreement);
                }
            }
        }
        agreement
    }
}
