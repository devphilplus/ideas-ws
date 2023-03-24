use log::{
    info,
    debug,
    error
};

use serde::{
    Serialize,
    Deserialize
};

use configuration::ApplicationConfiguration;
use common::{
    tenant::Tenant,
    user::User
};

use crate::data::Data;



#[derive(Debug)]
pub enum TenantsError {
    ToBeImplemented(String),
    ConfigurationError
}

#[derive(Debug, Clone)]
pub struct Tenants {
    cfg: ApplicationConfiguration,
    data: Data
}


impl Tenants {

    pub fn new(
        cfg: ApplicationConfiguration
    ) -> Result<Self, TenantsError> {
        if let Ok(data) = Data::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data
            });
        }

        return Err(TenantsError::ConfigurationError);
    }

    pub async fn tenant_by_name(
        &self,
        name: &str
    ) -> Result<Tenant, TenantsError> {
        info!("Clients::tenant_by_name()");

        match self.data.tenant_by_name(name).await {
            Err(e) => {
                error!("unable to retrieve tenant: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_by_name()")));
            }
            Ok(tenant) => {
                return Ok(tenant);
            }
        }
    }

    pub async fn tenant_add(
        &self,
        id: uuid::Uuid,
        name: &str,
        slug: &str,
        description: &str
    ) -> Result<(), TenantsError> {
        info!("Tenants::tenant_add()");

        match self.data.tenant_add(
            &id,
            &name,
            &slug,
            &description
        ).await {
            Err(e) => {
                error!("unable to retrieve tenant: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_add()")));
            }
            Ok(client) => {
                return Ok(client);
            }
        }
    }
}