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
    fn automatic_agreement(&self, mut agreement: Agreement) -> Agreement {

        //private key should be created from the user identity and the agreement and a cobination of other factors and then we sign the contract to get a signature

        let signature = Signature {
            agrees_to: Box::new(agreement.clone()),
            value: String::from("here is my signature"),
        };

        let new_agreement = (Some(signature), None);
        agreement.proof_of_agreement = Some(new_agreement.clone());
        Agreement {
            proof_of_agreement: Some(new_agreement),
            ..agreement
        }
    }
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
                //private key should be created from the user identity and the agreement and a cobination of other factors and then we sign the contract to get a signature
        let signature = Signature {
            agrees_to: Box::new(agreement.clone()),
            value: String::from("I solely and independently agree on this"),
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
