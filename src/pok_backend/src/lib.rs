#[macro_use]
extern crate serde;
use std::cell::RefCell;

use agreement::Agreement;
use candid::Principal;
use chrono::prelude::*;
use helpers::ToUser;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap, Cell, DefaultMemoryImpl, Vec as VecStructure,
};
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
        Utc::now().to_string(),
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
    String::from("Proof of agreement is in a great working condition")
}
ic_cdk::export_candid!();
