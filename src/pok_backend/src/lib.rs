use agreement::Agreement;
use candid::Principal;
use helpers::ToUser;
use user::{CreateAgreement, User};
mod agreement;
mod helpers;
mod signature;
mod user;

impl ToUser for Principal {
    fn principal_to_user(self) -> User {
        User { identity: self }
    }
}

fn _create_new_agreement(terms: Vec<String>, with_user: Principal) -> Agreement {
    let creator = Principal::principal_to_user(ic_cdk::caller());
    creator.new_agreement(
        terms,
        String::from("new date"),
        Principal::principal_to_user(with_user),
    )
}
