use common::user::User;
use log::{
    info,
    debug,
    error
};
use std::str::FromStr;

use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager, 
    ManagerConfig, 
    Pool, 
    RecyclingMethod
};
use tokio_postgres::{
    NoTls,
    // row::Row
};
use tokio_postgres::config::{ Config };

use chrono::prelude::*;

use configuration::{
    ApplicationConfiguration,
    ProviderType
};
use common::client::Client;



#[derive(Debug)]
pub enum DataError {
    ToBeImplemented(String),
    ConfigurationError,
    DatabaseError
}

#[derive(Debug, Clone)]
pub struct Data {
    pool: Pool
}

impl Data {

    pub fn new(cfg: &ApplicationConfiguration) -> Result<Self, DataError> {
        for p in &cfg.providers {
            if matches!(p.provider_type, ProviderType::Postgres) {
                for url in &p.url {
                    match Config::from_str(&url) {
                        Err(e) => {
                            debug!("error: {:?}", e);
                            return Err(DataError::ConfigurationError);
                        }
                        Ok(c) => {
                            let mgr = Manager::from_config(
                                c, 
                                NoTls, 
                                ManagerConfig { recycling_method: RecyclingMethod::Fast }
                            );
                            match Pool::builder(mgr)
                                .max_size(4)
                                .build() {
                                    Err(e) => {
                                        debug!("error: {:?}", e);
                                        return Err(DataError::ToBeImplemented(String::from("new")));
                                    }
                                    Ok(pool) => {
                                        return Ok(Self {
                                            pool: pool
                                        });
                                    }
                                }
                        }
                    }
                }
            }
        }

        return Err(DataError::ConfigurationError);
    }

    pub async fn client_by_name(
        &self,
        name: &str
    ) -> Result<Client, DataError> {
        info!("Data::client_by_name()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from client.client_by_name($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &name
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(row) => {
                debug!("row: {:?}", row);

                let id: uuid::Uuid = row.get("id");
                let active: bool = row.get("active");
                let name: String = row.get("name");

                return Ok(Client::new(
                    &id,
                    &active,
                    &name
                ));
            }
        }
    }


    pub async fn clients(
        &self
    ) -> Result<Vec<Client>, DataError> {
        info!("Data::clients()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from client.clients_fetch()"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query(
            &stmt,
            &[
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(rows) => {
                let clients = rows.iter().map(|r| {
                    let id: uuid::Uuid = r.get("id");
                    let active: bool = r.get("active");
                    let name: String = r.get("name");

                    return Client::new(
                        &id,
                        &active,
                        &name
                    );
                }).collect();
                return Ok(clients);
            }
        }
    }


    pub async fn client_add(
        &self,
        id: &uuid::Uuid,
        name: &str,
        slug: &str,
        description: &str
    ) -> Result<(), DataError> {
        info!("Data::client_add()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from client.client_add($1, $2, $3, $4)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &id,
                &name,
                &slug,
                &description
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(row) => {
                debug!("row: {:?}", row);

                let id: uuid::Uuid = row.get(0);
                let name: String = row.get(1);

                return Ok(());
            }
        }

    }

    pub async fn users(
        &self,
        client_id: &uuid::Uuid
    ) -> Result<Vec<User>, DataError> {
        info!("Data::users()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from iam.user_clients_by_client($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query(
            &stmt,
            &[
                &client_id
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(rows) => {
                debug!("rows: {:?}", rows);

                let results = rows.iter().map(|r| {
                    let user_id: uuid::Uuid = r.get("user_id");
                    let active: bool = r.get("active");
                    let email: String = r.get("email");

                    return User::new(
                        &user_id,
                        &active,
                        &email,
                        &"",
                        &"",
                        &""
                    );
                }).collect();

                return Ok(results);
            }
        }

    }
}