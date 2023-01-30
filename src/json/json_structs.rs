use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct AccountList {
    pub(crate) account_list: Vec<Account>,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct Account {
    pub(crate) website: String,
    pub(crate) username: String,
    pub(crate) password: Vec<u8>,
    pub(crate) nonce: Vec<u8>,
}
