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
        cfg: ApplicationConfiguration,
        data: data::Data
    ) -> Self {
        // if let Ok(data) = OrganizationsData::new(&cfg) {
        //     return Ok(Self {
        //         cfg: cfg,
        //         data: data
        //     });
        // }

        // return Err(Error::ConfigurationError(String::from("unable to create data object")));
        let org_data = OrganizationsData::new(data);
        return Self {
            cfg: cfg,
            data: org_data
        };
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

    pub async fn set_active(
        &self,
        organization_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), Error> {
        info!("Organizations::set_active()");

        match self.data.organization_set_active(
            &organization_id,
            &active
        ).await {
            Err(e) => {
                error!("unable to set organization active status: {:?}", e);
                return Err(Error::ToBeImplemented(String::from("Organizations::set_active()")));
            }
            Ok(()) => {
                return Ok(());
            }
        }
    }
}