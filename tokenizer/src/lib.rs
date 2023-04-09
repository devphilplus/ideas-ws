use log::{
    info,
    error,
    debug
};

use chrono::{prelude::*, Duration};

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
use std::collections::BTreeMap;



#[derive(Debug)]
pub struct Claims {
    email: String,
    issued: chrono::DateTime<Utc>,
    expiry: chrono::DateTime<Utc>
}

impl Claims {

    pub fn new(
        email: &str,
        issued: &chrono::DateTime<Utc>,
        expiry: &chrono::DateTime<Utc>
    ) -> Self {
        return Self {
            email: String::from(email),
            issued: issued.clone(),
            expiry: expiry.clone()
        };
    }

    pub fn email(&self) -> String {
        return self.email.clone();
    }

    pub fn issued_at(&self) -> chrono::DateTime<Utc> {
        return self.issued;
    }

    pub fn expiry(&self) -> chrono::DateTime<Utc> {
        return self.expiry;
    }
}


pub enum TokenError {
    HashError,
    SigningError,
    ToBeImplementedError
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    secret: String
}

impl Tokenizer {
    pub fn new(secret: &str) -> Self {
        return Self {
            secret: String::from(secret)
        };
    }

    /**
     * TODO: implement a token prefix for some obfuscation
     * TODO: implement some simple encryption for obfuscation
     */
    pub fn generate(
        &self,
        email: &str
    ) -> Result<String, TokenError> {
        let mut claims = BTreeMap::new();

        claims.insert("email", email);

        // iss (issuer)

        // iat (issued at)
        let iat = Utc::now().to_rfc3339();
        claims.insert("iat", iat.as_str());

        // exp (expiry)
        let exp = (Utc::now() + Duration::hours(1)).to_string();
        claims.insert("exp", exp.as_str());

        match <Hmac<Sha256>>::new_from_slice(self.secret.as_bytes()) {
            Err(e) => {
                error!("unable to generate key: {:?}", e);
                return Err(TokenError::HashError);
            }
            Ok(key) => {
                match claims.sign_with_key(&key) {
                    Err(e) => {
                        error!("unable to sign claims: {:?}", e);
                        return Err(TokenError::SigningError);
                    }
                    Ok(token) => {
                        return Ok(token);
                    }
                }
            }
        }
    }

    pub fn is_valid(
        &self,
        token: &str
    ) -> bool {
        info!("Tokenizer::is_valid()");

        if token.is_empty() {
            error!("cannot verify an empty token");
            return false;
        } else {
            match <Hmac<Sha256>>::new_from_slice(
                self.secret.as_bytes()
            ) {
                Err(e) => {
                    error!("unable to generate key: {:?}", e);
                    return false;
                }
                Ok(key) => {
                    debug!("key: {:?}", key);
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

    pub fn get_claims(&self, token: &str) -> Result<Claims, TokenError> {
        if let Ok(key) = <Hmac<Sha256>>::new_from_slice(
            self.secret.as_bytes()
        ) {
            let result: Result<BTreeMap<String, String>, error::Error> = token.verify_with_key(&key);
            if let Ok(claims) = result {
                debug!("claims: {:?}", claims);

                let issued_at = chrono::DateTime::parse_from_rfc2822(
                    claims.get("iat").unwrap()
                ).unwrap().with_timezone(&chrono::Utc);

                let expiry = chrono::DateTime::parse_from_rfc2822(
                    claims.get("exp").unwrap()
                ).unwrap().with_timezone(&chrono::Utc);

                return Ok(Claims::new(
                    claims.get("email").unwrap(),
                    &issued_at,
                    &expiry
                    
                ));
            } else {
                error!("unable to get claims");
                return Err(TokenError::ToBeImplementedError);
            }
        } else {
            error!("unable to get claims");
            return Err(TokenError::ToBeImplementedError);
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
