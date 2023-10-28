use std::io::Write;
use std::string::FromUtf8Error;
use sha256::digest;
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
            password: hash(password, None)
        }
    }
}

// **
pub fn hash(input: String, salt: Option<String>) -> String {
    let mut input = input;
    if let Some(salt) = salt {
        input += salt.as_str();
    }
    digest(input)
}


#[cfg(test)]
mod tests {
    use super::hash;
    #[test]
    fn hash_test() {
        let result = hash("testing".to_string(), None);
        println!("{}", result);
        assert_eq!(result, "cf80cd8aed482d5d1527d7dc72fceff84e6326592848447d2dc0b0e87dfc9a90".to_string());
    }
}