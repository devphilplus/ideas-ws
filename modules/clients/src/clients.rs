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
use common::{client::Client, user::User};
use data::Data;
// use crate::data::client::ClientData;


#[derive(Debug)]
pub enum ClientsError {
    ToBeImplemented(String),
    ConfigurationError
}

#[derive(Debug, Clone)]
pub struct Clients {
    cfg: ApplicationConfiguration,
    data: crate::data::client::ClientData
}


impl Clients {

    pub fn new(
        cfg: ApplicationConfiguration,
        data: Data
    ) -> Result<Self, ClientsError> {
        // if let Ok(data) = Data::new(&cfg) {
        //     return Ok(Self {
        //         cfg: cfg,
        //         data: data
        //     });
        // }

        // return Err(ClientsError::ConfigurationError);
        let client_data = crate::data::client::ClientData::new(data);
        return Self {
            cfg: cfg,
            data: client_data
        };
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

    pub async fn clients(&self) -> Result<Vec<Client>, ClientsError> {
        info!("Clients::clients()");

        match self.data.clients().await {
            Err(e) => {
                error!("unable to retrieve clients: {:?}", e);
                return Err(ClientsError::ToBeImplemented(String::from("Clients::clients()")));
            }
            Ok(clients) => {
                return Ok(clients);
            }
        }
    }

    pub async fn users(
        &self,
        client_id: &uuid::Uuid
    ) -> Result<Vec<User>, ClientsError> {
        info!("Clients::users()");

        match self.data.users(
            &client_id
        ).await {
            Err(e) => {
                error!("unable to retrieve users: {:?}", e);
                return Err(ClientsError::ToBeImplemented(String::from("Clients::users()")));
            }
            Ok(users) => {
                return Ok(users);
            }
        }

    }
}