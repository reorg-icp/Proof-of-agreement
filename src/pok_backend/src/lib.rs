#[macro_use]
extern crate serde;
use std::cell::RefCell;

use agreement::Agreement;
use candid::Principal;
use chrono::prelude::*;
use helpers::ToUser;
use ic_cdk::api::time;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap, Cell, DefaultMemoryImpl, Vec as VecStructure,
};
use lamport::{hash, verify};
use user::{Agree, CreateAgreement, User};

mod agreement;
mod helpers;
mod lamport;
mod signature;
mod user;

//Memory implementations
type Memory = VirtualMemory<DefaultMemoryImpl>;

type IdCell = Cell<u64, Memory>;

thread_local! {
        static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USERS: RefCell<BTreeMap<u64,User,Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

        static AGREEMENTS: RefCell<BTreeMap<u64,Agreement,Memory>> = RefCell::new(
        BTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )



    );
        static USER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a  User counter")
    );
    static AGREEMENT_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create an Agreements  counter")
    );


}

impl ToUser for Principal {
    fn principal_to_user(name: String) -> User {
        User {
            identity: name,
            agreements: vec![],
        }
    }
}

fn _create_new_agreement(terms: Vec<String>, with_user: String, id: u64) -> Agreement {
    let creator = Principal::principal_to_user(String::from("aMSCHEL"));

    let agreement = creator.clone().new_agreement(
        terms,
        time().to_string(),
        Principal::principal_to_user(with_user),
        id,
    );
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
        let agreement = _create_new_agreement(terms, String::from("God"), 1);
        let amschel_agrees = _agree_to_agreement(String::from("God"), agreement);
        dbg!(amschel_agrees.proof_of_agreement.unwrap().0.unwrap().value);
    }
    #[test]
    fn _agree_to_agreement_works() {}
}

// Internet computer functions here

#[ic_cdk::query]
fn check_status() -> String {
    String::from("We are live")
}

#[ic_cdk::update]

fn initiate_agreement(terms: Vec<String>, with_user: String) -> Result<Agreement, Error> {
    let id = AGREEMENT_ID_COUNTER.with(|counter| {
        let counter_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(counter_value + 1);
        counter_value
    });

    let agreement = _create_new_agreement(terms, with_user, id);

    match AGREEMENTS.with(|db| db.borrow_mut().insert(id, agreement.clone())) {
        Some(_) => {
            let new_agreement = AGREEMENTS
                .with(|storage| storage.borrow_mut().get(&agreement.clone().id))
                .unwrap();
            Ok(new_agreement)
        }
        None => {
            let new_agreement = AGREEMENTS
                .with(|storage| storage.borrow_mut().get(&agreement.clone().id))
                .unwrap();
            Ok(new_agreement)
        }
    }
}

#[ic_cdk::update]

fn signup_user() -> String {
    let id = USER_ID_COUNTER.with(|counter| {
        let counter_value = *counter.borrow().get();
        let _ = counter.borrow_mut().set(counter_value + 1);
        counter_value
    });
    let user = User {
        agreements: vec![],
        identity: ic_cdk::caller().to_string(),
    };

    match USERS.with(|db| db.borrow_mut().insert(id, user.clone())) {
        Some(_) => format!("User {} created", user.identity),
        None => format!("User {} created", user.identity),
    }
}

#[ic_cdk::update]

fn agree_to(agreement_id: u64) -> Result<Agreement, Error> {
    //We are supposed to sign and store the update in stable storage

    let initial_agreement = AGREEMENTS.with(|storage| storage.borrow_mut().get(&agreement_id));
    match initial_agreement {
        Some(agreement) => {
            let signed_agreement =
                _agree_to_agreement(ic_cdk::caller().to_string(), agreement.clone());

            match AGREEMENTS.with(|storage| {
                storage
                    .borrow_mut()
                    .insert(agreement.clone().id, signed_agreement)
            }) {
                Some(agreement) => {
                    let new_agreement = AGREEMENTS
                        .with(|storage| storage.borrow_mut().get(&agreement.clone().id))
                        .unwrap();
                    Ok(new_agreement)
                }
                None => {
                    let new_agreement = AGREEMENTS
                        .with(|storage| storage.borrow_mut().get(&agreement.clone().id))
                        .unwrap();
                    Ok(new_agreement)
                }
            }
        }
        None => Err(Error::NotFound {
            msg: format!("That agreement was not found"),
        }),
    }
}

#[ic_cdk::update]

fn verify_signatures(agreement_id: u64) -> Result<bool, Error> {
    let agreement = AGREEMENTS.with(|storage| storage.borrow_mut().get(&agreement_id));
    match agreement {
        Some(agreement) => {
            //Reconstruct the message
            let mut message: String = String::new();
            for term in agreement.clone().terms.iter() {
                message.push_str(term);
            }
            //extract the signatures and public keys and then verify

            let signature1 = agreement
                .clone()
                .proof_of_agreement
                .unwrap()
                .0
                .unwrap()
                .value;
            let signature2 = agreement
                .clone()
                .proof_of_agreement
                .unwrap()
                .1
                .unwrap()
                .value;
            let key1 = agreement.clone().public_keys.unwrap().0.unwrap();
            let key2 = agreement.clone().public_keys.unwrap().1.unwrap();
            let signature_one_is_valid = verify(hash(&message.as_str()), &signature1, &key1);
            let signature_two_is_valid = verify(hash(&message.as_str()), &signature2, &key2);
            Ok(signature_one_is_valid && signature_two_is_valid)

            //do something
        }
        None => Err(Error::NotFound {
            msg: format!("That agreement was not found"),
        }),
    }
}
#[derive(candid::CandidType, Deserialize, Serialize, Debug)]
enum Error {
    NotFound { msg: String },
}

ic_cdk::export_candid!();
