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

use crate::data::tenants::TenantsData;



#[derive(Debug)]
pub enum TenantsError {
    ToBeImplemented(String),
    ConfigurationError
}

#[derive(Debug, Clone)]
pub struct Tenants {
    cfg: ApplicationConfiguration,
    data: crate::data::tenants::TenantsData
}


impl Tenants {

    pub fn new(
        cfg: ApplicationConfiguration
    ) -> Result<Self, TenantsError> {
        if let Ok(data) = TenantsData::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data
            });
        }

        return Err(TenantsError::ConfigurationError);
    }

    /// retrieve tenants
    pub async fn tenants(&self) -> Result<Vec<Tenant>, TenantsError> {
        info!("Tenants::tenants()");

        match self.data.tenants_fetch().await {
            Err(e) => {
                error!("unable to retrieve tenants: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenants()")));
            }
            Ok(tenants) => {
                return Ok(tenants);
            }
        }
    }

    // /// retrieve members
    // pub fn members(&self) -> crate::members::Members {
    //     return crate::members::Members::new(&self.data);
    // }

    /// retrieve tenant by id
    pub async fn tenant_by_id(
        &self,
        tenant_id: &uuid::Uuid
    ) -> Result<Tenant, TenantsError> {
        info!("Tenants::tenant_by_name()");

        match self.data.tenant_by_id(&tenant_id).await {
            Err(e) => {
                error!("unable to retrieve tenant: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_by_id()")));
            }
            Ok(tenant) => {
                return Ok(tenant);
            }
        }
    }

    /// retrieve tenant by name
    pub async fn tenant_by_name(
        &self,
        name: &str
    ) -> Result<Tenant, TenantsError> {
        info!("Tenants::tenant_by_name()");

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

    /// retrieve tenant by slug
    pub async fn tenant_by_slug(
        &self,
        slug: &str
    ) -> Result<Tenant, TenantsError> {
        info!("Tenants::tenant_by_slug()");

        match self.data.tenant_by_slug(slug).await {
            Err(e) => {
                error!("unable to retrieve tenant: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_by_slug()")));
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
                error!("unable to add tenant: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_add()")));
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }

    pub async fn tenant_update(
        &self,
        id: uuid::Uuid,
        name: &str,
        slug: &str,
        description: &str
    ) -> Result<(), TenantsError> {
        info!("Tenants::tenant_update()");

        match self.data.tenant_update(
            &id,
            &name,
            &slug,
            &description
        ).await {
            Err(e) => {
                error!("unable to update tenant: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_update()")));
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }

    pub async fn tenant_set_active(
        &self,
        tenant_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), TenantsError> {
        info!("Tenants::tenant_set_active()");

        match self.data.tenant_set_active(
            &tenant_id,
            &active
        ).await {
            Err(e) => {
                error!("unable to set tenant active status: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_set_active()")));
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }

    pub async fn tenant_users_fetch(
        &self,
        tenant_id: &uuid::Uuid
    ) -> Result<Vec<common::user::User>, TenantsError> {
        info!("Tenants::tenant_users_fetch()");

        match self.data.tenant_users_fetch(
            &tenant_id
        ).await {
            Err(e) => {
                error!("unable to retrieve tenant users: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_users_fetch()")));
            }
            Ok(users) => {
                return Ok(users);
            }
        }
    }

    pub async fn tenant_default_fetch(
        &self,
        tenant_id: &uuid::Uuid
    ) -> Result<common::tenant::Tenant, TenantsError> {
        info!("Tenants::tenant_default_fetch()");

        match self.data.tenant_default_fetch(
            &tenant_id
        ).await {
            Err(e) => {
                error!("unable to retrieve default tenant: {:?}", e);
                return Err(TenantsError::ToBeImplemented(String::from("Tenants::tenant_default_fetch()")));
            }
            Ok(tenant) => {
                return Ok(tenant);
            }
        }
    }
}