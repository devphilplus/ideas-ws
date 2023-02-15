use log::{
    info,
    debug,
    error
};


use configuration::ApplicationConfiguration;

use crate::data::DataError;


#[derive(Debug)]
pub enum AuthError {
    ToBeImplemented(String)
}


pub struct Auth {
    cfg: ApplicationConfiguration,
    data: crate::data::Data
}

impl Auth {

    pub fn new(cfg: ApplicationConfiguration) -> Self {
        let data = crate::data::Data::new();

        return Self {
            cfg: cfg,
            data: data
        };
    }

    pub fn register(&self, token: &str, email: &str) -> Result<(), AuthError> {
        match self.data.register(token, email) {
            Err(e) => {
                match e {
                    DataError::ToBeImplemented(method) => {
                        debug!("method not implemented: Data::{}", method);
                        return Err(AuthError::ToBeImplemented(String::from("register")))
                    }
                }
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }
}