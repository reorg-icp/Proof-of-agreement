use agreement::Agreement;
use candid::Principal;
use chrono::prelude::*;
use helpers::ToUser;
use user::{Agree, CreateAgreement, User};
mod agreement;
mod helpers;
mod lamport;
mod signature;
mod user;

impl ToUser for Principal {
    fn principal_to_user(name: String) -> User {
        User { identity: name }
    }
}

fn _create_new_agreement(terms: Vec<String>, with_user: String) -> Agreement {
    let creator = Principal::principal_to_user(String::from("aMSCHEL"));

    let agreement =
        creator
            .clone()
            .new_agreement(terms, Utc::now(), Principal::principal_to_user(with_user));
    creator.automatic_agreement(agreement)
}
fn _agree_to_agreement(user: String, agreement: Agreement) -> Agreement {
    let agreeing_party = Principal::principal_to_user(user);
    agreeing_party.agree(agreement)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn agreement_btwn_god_and_man() {
        let terms: Vec<String> = vec![
        "I am the Lord thy God".to_string(),
        "Thou shalt have no other gods before me".to_string(),
        "Thou shalt not make unto thee any graven image".to_string(),
        "Thou shalt not take the name of the Lord thy God in vain".to_string(),
        "Remember the sabbath day, to keep it holy".to_string(),
        "Honour thy father and thy mother".to_string(),
        "Thou shalt not kill".to_string(),
        "Thou shalt not commit adultery".to_string(),
        "Thou shalt not steal".to_string(),
        "Thou shalt not bear false witness against thy neighbour".to_string(),
        "Thou shalt not covet thy neighbour's house".to_string(),
        "Thou shalt not covet thy neighbour's wife, nor his manservant, nor his maidservant, nor his ox, nor his ass, nor any thing that is thy neighbour's".to_string(),
    ];
        let agreement = _create_new_agreement(terms, String::from("God"));
        let amschel_agrees = _agree_to_agreement(String::from("God"), agreement);
        dbg!(amschel_agrees.proof_of_agreement.unwrap().0.unwrap().value);
    }
    #[test]
    fn _agree_to_agreement_works() {}
}
