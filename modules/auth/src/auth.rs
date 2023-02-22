use std::fmt::Display;

use log::{
    info,
    debug,
    error
};

use chrono::prelude::*;
use serde::{
    Serialize,
    Deserialize
};


use configuration::ApplicationConfiguration;
use mailer::Mailer;

use crate::data::{
    DataError,
    Data
};


#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationInfo {
    pub id: uuid::Uuid,
    pub email: String,
    pub created: DateTime<Utc>
}



#[derive(Debug)]
pub enum AuthError {
    ToBeImplemented(String),
    ConfigurationError,
    MailerError
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
        if let Ok(data) = Data::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data,
                mailer: mailer
            });
        }

        return Err(AuthError::ConfigurationError);
    }

    /// register user and then sends an email containing the link to complete
    /// the registration process
    pub async fn register(&self, id: &uuid::Uuid, email: &str) -> Result<(), AuthError> {
        match self.data.register(id, email).await {
            Err(e) => {
                error!("unable to register: {:?}", e);
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
                let body = format!("<p>Please click on the link to \
                    complete the registration: <a href=\"{base_url}/sign-up/continue/{token}\">{base_url}/sign-up/continue/{token}</a></p>",
                    base_url = self.cfg.base_url,
                    token = token
                );
                if let Err(e) = self.mailer.send(
                    &self.cfg.mailer.defaults.from,
                    &email,
                    "Registration", &body) {
                        error!("failed to send email: {:?}", e);
                        return Err(AuthError::MailerError)
                }
                return Ok(());
            }
        }
    }

    /// retrieve registration details
    pub async fn get_registration_info(&self, token: &str) -> Result<RegistrationInfo, AuthError> {
        match self.data.get_registration_info(token).await {
            Err(e) => {
                return Err(AuthError::ToBeImplemented(String::from("get_registration_info")));
            }
            Ok(result) => {
                debug!("result: {:?}", result);
                return Ok(RegistrationInfo { 
                    id: result.id,
                    email: result.email,
                    created: result.created
                });
            }
        }
    }
}