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

use crate::data::Data;


#[derive(Debug)]
pub enum UsersError {
    ToBeImplemented(String),
    ConfigurationError,
    MailerError,
    ValidationError
}


#[derive(Debug, Clone)]
pub struct Users {
    cfg: ApplicationConfiguration,
    data: crate::data::Data,
    mailer: Mailer
}

impl Users {

    pub fn new(
        cfg: ApplicationConfiguration,
        mailer: Mailer
    ) -> Result<Self, UsersError> {
        if let Ok(data) = Data::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data,
                mailer: mailer
            });
        }

        return Err(UsersError::ConfigurationError);
    }

    pub async fn user_set_active(
        &self,
        user_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), UsersError> {
        info!("Users::user_set_active()");

        match self.data.user_set_active(
            &user_id,
            &active
        ).await {
            Err(e) => {
                error!("unable to set user active status: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_set_active")));
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }

    pub async fn user_set_password(
        &self,
        user_id: &uuid::Uuid,
        password: &str
    ) -> Result<(), UsersError> {
        info!("Users::user_set_password()");

        match self.data.user_set_password(
            &user_id,
            &password
        ).await {
            Err(e) => {
                error!("unable to set user password: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_set_active")));
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }
}