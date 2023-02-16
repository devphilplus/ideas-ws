use std::fmt::Display;

use log::{
    info,
    debug,
    error
};


use configuration::ApplicationConfiguration;

use crate::data::{DataError, Data};


#[derive(Debug)]
pub enum AuthError {
    ToBeImplemented(String),
    ConfigurationError
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


#[derive(Debug, Clone)]
pub struct Auth {
    cfg: ApplicationConfiguration,
    data: crate::data::Data
}

impl Auth {

    pub fn new(cfg: ApplicationConfiguration) -> Result<Self, AuthError> {
        if let Ok(data) = crate::data::Data::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data
            });
        }

        return Err(AuthError::ConfigurationError);
    }

    pub fn register(&self, token: &uuid::Uuid, email: &str) -> Result<(), AuthError> {
        match self.data.register(token, email) {
            Err(e) => {
                match e {
                    DataError::ToBeImplemented(method) => {
                        debug!("method not implemented: Data::{}", method);
                        return Err(AuthError::ToBeImplemented(String::from("register")))
                    }
                    DataError::ConfigurationError => {
                        return Err(AuthError::ConfigurationError);
                    }
                }
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }
}