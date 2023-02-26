use std::collections::BTreeMap;

use log::{
    info,
    error,
    debug
};

use chrono::prelude::*;

use hmac::{
    Hmac,
    Mac
};

use jwt::{
    SignWithKey,
    VerifyWithKey,
    error
};
use sha2::Sha256;




pub struct Claims {
    email: String,
    issued: chrono::DateTime<Utc>
}

impl Claims {

}


pub struct Tokenizer {
    secret: String
}

impl Tokenizer {
    pub fn new(secret: &str) -> Self {
        return Self {
            secret: String::from(secret)
        };
    }

    pub fn generate(
        &self,
        email: &str
    ) -> Result<String, String> {
        let mut claims = BTreeMap::new();

        claims.insert("email", email);

        match <Hmac<Sha256>>::new_from_slice(self.secret.as_bytes()) {
            Err(e) => {
                error!("unable to generate key: {:?}", e);
                return Err(String::from("unable to generate key"));
            }
            Ok(key) => {
                match claims.sign_with_key(&key) {
                    Err(e) => {
                        error!("unable to sign claims: {:?}", e);
                        return Err(String::from("unable to sign claims"));
                    }
                    Ok(token) => {
                        return Ok(token);
                    }
                }
            }
        }
    }

    pub fn validate(
        &self,
        token: &str
    ) -> bool {
        match <Hmac<Sha256>>::new_from_slice(
            self.secret.as_bytes()
        ) {
            Err(e) => {
                error!("unable to generate key: {:?}", e);
                return false;
            }
            Ok(key) => {
                let result: Result<BTreeMap<String, String>, error::Error> = token.verify_with_key(&key);
                match result {
                    Err(e) => {
                        error!("unable to verify token: {:?}", e);
                        return false;
                    }
                    Ok(_) => {
                        return true;
                    }
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let tokenizer = Tokenizer::new("testing");
        match tokenizer.generate("testing@mailinator.com") {
            Err(e) => {
                assert!(false, "unable to generate token");
            }
            Ok(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_validate() {
        let tokenizer = Tokenizer::new("testing");
        assert!(true);
    }
}
