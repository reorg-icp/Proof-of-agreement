use crate::agreement::Agreement;
#[derive(Clone)]
pub struct Signature {
    pub agrees_to: Box<Agreement>,

    pub value: String,
}
