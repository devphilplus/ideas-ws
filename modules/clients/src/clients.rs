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
use common::client::Client;

use crate::data::Data;


#[derive(Debug)]
pub enum ClientsError {
    ToBeImplemented(String),
    ConfigurationError
}

#[derive(Debug, Clone)]
pub struct Clients {
    cfg: ApplicationConfiguration,
    data: Data
}


impl Clients {

    pub fn new(
        cfg: ApplicationConfiguration
    ) -> Result<Self, ClientsError> {
        if let Ok(data) = Data::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data
            });
        }

        return Err(ClientsError::ConfigurationError);
    }

    pub async fn client_by_name(
        &self,
        name: &str
    ) -> Result<Client, ClientsError> {
        info!("Clients::client_by_name()");

        match self.data.client_by_name(name).await {
            Err(e) => {
                error!("unable to retrieve client: {:?}", e);
                return Err(ClientsError::ToBeImplemented(String::from("Clients::client_by_name()")));
            }
            Ok(client) => {
                return Ok(client);
            }
        }
    }

    pub async fn client_add(
        &self,
        id: uuid::Uuid,
        name: &str,
        slug: &str,
        description: &str
    ) -> Result<(), ClientsError> {
        info!("Clients::client_add()");

        match self.data.client_add(
            &id,
            &name,
            &slug,
            &description
        ).await {
            Err(e) => {
                error!("unable to retrieve client: {:?}", e);
                return Err(ClientsError::ToBeImplemented(String::from("Clients::client_add()")));
            }
            Ok(client) => {
                return Ok(client);
            }
        }
    }
}