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
    Error
};

use crate::data::organizations::OrganizationsData;


#[derive(Debug, Clone)]
pub struct Organizations {
    cfg: ApplicationConfiguration,
    data: crate::data::organizations::OrganizationsData
}



impl Organizations {

    pub fn new(
        cfg: ApplicationConfiguration
    ) -> Result<Self, Error> {
        if let Ok(data) = OrganizationsData::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data
            });
        }

        return Err(Error::ConfigurationError(String::from("unable to create data object")));
    }

    pub async fn add(
        &self,
        tenant_id: &uuid::Uuid,
        organization_id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> Result<(), Error> {
        info!("Organizations::add()");

        match self.data.add_organization(
            &tenant_id,
            &organization_id,
            &name,
            &description
        ).await {
            Err(e) => {
                error!("unable to add organization record: {:?}", e);
                return Err(Error::ToBeImplemented(String::from("Organizations::add()")));
            }
            Ok(()) => {
                return Ok(());
            }
        }
    }
}