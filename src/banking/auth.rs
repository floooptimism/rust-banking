use crate::banking::types::ID;

pub struct Credential {
    account_id: ID,
    username: String,
    password: String
}

impl Credential {
    pub fn new(id: ID, username: String, password: String) -> Credential {
        Credential {
            account_id: id,
            username,
            password
        }
    }
}