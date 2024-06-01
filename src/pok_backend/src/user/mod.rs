use crate::agreement::{self, Agreement, ProofOfAgreement};
use crate::lamport::{hash, random_private_key, sign};
use crate::signature::{self, Signature};

use candid::Principal;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
#[derive(Clone, Debug)]
pub struct User {
    pub identity: String,
    pub agreements: Vec<u64>,
}
pub trait CreateAgreement {
    fn new_agreement(
        self,
        terms: Vec<String>,
        date: DateTime<Utc>,
        with_user: User,
        id: u64,
    ) -> Agreement;
}
pub trait Agree {
    fn agree(self, agreement: Agreement) -> Agreement;
    fn automatic_agreement(&self, mut agreement: Agreement) -> Agreement {
        //private key should be created from the user identity and the agreement and a cobination of other factors and then we sign the contract to get a signature
        let privateKey = random_private_key(
            agreement.clone().by_user.identity,
            agreement.clone(),
            String::from("example nounce"),
        );
        let mut terms_string: String = String::new();
        for term in agreement.clone().terms.iter() {
            terms_string.push_str(term);
        }
        let generated_signature = sign(hash(&terms_string), &privateKey);
        //private key should be created from the user identity and the agreement and a cobination of other factors and then we sign the contract to get a signature
        let signature = Signature {
            agrees_to: Box::new(agreement.clone()),
            value: generated_signature,
        };

        let new_agreement = (Some(signature), None);
        agreement.proof_of_agreement = Some(new_agreement.clone());
        Agreement {
            proof_of_agreement: Some(new_agreement),
            ..agreement.clone()
        }
    }
}
impl CreateAgreement for User {
    fn new_agreement(
        self,
        terms: Vec<String>,
        date: DateTime<Utc>,
        with_user: User,
        id: u64,
    ) -> Agreement {
        Agreement {
            by_user: self,
            with_user,
            terms,
            date,
            id,
            proof_of_agreement: None,
        }
    }
}

impl Agree for User {
    fn agree(self, mut agreement: Agreement) -> Agreement {
        let privateKey = random_private_key(
            self.identity,
            agreement.clone(),
            String::from("example nounce"),
        );
        let mut terms_string: String = String::new();
        for term in agreement.clone().terms.iter() {
            terms_string.push_str(term);
        }
        let generated_signature = sign(hash(&terms_string), &privateKey);
        //private key should be created from the user identity and the agreement and a cobination of other factors and then we sign the contract to get a signature
        let signature = Signature {
            agrees_to: Box::new(agreement.clone()),
            value: generated_signature,
        };

        if let Some((first_sig_opt, second_sig_opt)) = &mut agreement.clone().proof_of_agreement {
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
