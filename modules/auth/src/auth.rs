use std::fmt::Display;

use log::{
    info,
    debug,
    error
};


use configuration::ApplicationConfiguration;
use mailer::Mailer;

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
    data: crate::data::Data,
    mailer: Mailer
}

impl Auth {

    pub fn new(
        cfg: ApplicationConfiguration,
        mailer: Mailer
    ) -> Result<Self, AuthError> {
        if let Ok(data) = crate::data::Data::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data,
                mailer: mailer
            });
        }

        return Err(AuthError::ConfigurationError);
    }

    pub async fn register(&self, token: &uuid::Uuid, email: &str) -> Result<String, AuthError> {
        match self.data.register(token, email).await {
            Err(e) => {
                match e {
                    DataError::ToBeImplemented(method) => {
                        debug!("method not implemented: Data::{}", method);
                        return Err(AuthError::ToBeImplemented(String::from("register")))
                    }
                    DataError::ConfigurationError => {
                        return Err(AuthError::ConfigurationError);
                    }
                    DataError::DatabaseError => {
                        return Err(AuthError::ConfigurationError)
                    }
                }
            }
            Ok(token) => {
                return Ok(token);
            }
        }
    }
}