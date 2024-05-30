use crate::user::User;

pub trait ToUser {
    fn principal_to_user(self) -> User;
}
