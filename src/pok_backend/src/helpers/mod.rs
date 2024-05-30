use crate::user::User;

pub trait ToUser {
    fn principal_to_user(name: String) -> User;
}
