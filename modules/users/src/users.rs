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
use common::{user::User, tenant::Tenant};

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

    pub async fn user_by_id(
        &self,
        id: &uuid::Uuid
    ) -> Result<User, UsersError> {
        info!("Users::user_by_id()");

        match self.data.by_id(
            &id
        ).await {
            Err(e) => {
                error!("unable to retrieve user: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_by_id")));
            }
            Ok(user) => {
                debug!("Users::user_by_id(): {:?}", user);
                return Ok(user);
            }
        }
    }

    pub async fn user_by_email(
        &self,
        email: &str
    ) -> Result<User, UsersError> {
        info!("Users::user_by_email()");

        match self.data.by_email(
            &email
        ).await {
            Err(e) => {
                error!("unable to retrieve user: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_by_email")));
            }
            Ok(user) => {
                debug!("Users::user_by_email(): {:?}", user);
                return Ok(user);
            }
        }
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

    pub async fn user_tenant_add(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid
    ) -> Result<(), UsersError> {
        info!("user_tenant_add");

        match self.data.user_tenant_add(
            &user_id,
            &tenant_id
        ).await {
            Err(e) => {
                error!("error: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_join_tenant")));
            }
            Ok(result) => {
                debug!("result: {:?}", result);
                return Ok(());
            }
        }
    }

    pub async fn user_tenant_set_active(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), UsersError> {
        info!("user_tenant_set_active");

        match self.data.user_tenant_set_active(
            &user_id,
            &tenant_id,
            &active
        ).await {
            Err(e) => {
                error!("error: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_tenant_set_active")));
            }
            Ok(result) => {
                debug!("result: {:?}", result);
                return Ok(());
            }
        }
    }

    pub async fn user_tenant_set_default(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid
    ) -> Result<(), UsersError> {
        info!("user_tenant_set_default");

        match self.data.user_tenant_set_default(
            &user_id,
            &tenant_id
        ).await {
            Err(e) => {
                error!("error: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_tenant_set_default")));
            }
            Ok(result) => {
                debug!("result: {:?}", result);
                return Ok(());
            }
        }
    }

    pub async fn user_tenants(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<Vec<Tenant>, UsersError> {
        info!("user_tenants");

        match self.data.user_tenants(&user_id).await {
            Err(e) => {
                error!("error: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_tenants")));
            }
            Ok(result) => {
                // debug!("//TODO result: {:?}", result);
                return Ok(result);
            }
        }
    }

    pub async fn user_tenant_default(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<Tenant, UsersError> {
        info!("user_tenant_default");

        match self.data.user_tenants_default(&user_id).await {
            Err(e) => {
                error!("error: {:?}", e);
                return Err(UsersError::ToBeImplemented(String::from("user_tenant_default")));
            }
            Ok(result) => {
                debug!("//TODO result: {:?}", result);
                return Ok(result);
            }
        }
    }
}