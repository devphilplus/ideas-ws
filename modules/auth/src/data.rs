use log::{
    info,
    debug,
    error
};

use std::str::FromStr;

use postgres_types::{ 
    ToSql
};
use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager, 
    ManagerConfig, 
    Pool, 
    RecyclingMethod,
    Client 
};
use tokio_postgres::{
    NoTls,
    // row::Row
};
use tokio_postgres::config::{ Config };

use rand::{
    thread_rng,
    Rng,
    distributions::{
        Alphanumeric
    }
};

use chrono::prelude::*;

use configuration::{
    ApplicationConfiguration,
    ProviderType
};


#[derive(Debug)]
pub enum DataError {
    ToBeImplemented(String),
    ConfigurationError,
    DatabaseError
}


#[derive(Debug)]
pub struct RegistrationInfo {
    pub token: String,
    pub email: String,
    pub created: DateTime<Utc>
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

    // async fn get_client(&self) -> Result<Client, DataError> {
    //     match self.pool.get().await {
    //         Err(e) => {
    //             error!("unable to retrieve database client: {:?}", e);
    //             return Err(DataError::DatabaseError);
    //         }
    //         Ok(client) => {
    //             return Ok(client);
    //         }
    //     }
    // }

    /// add record to registrations table in db and return token string
    pub async fn register(&self, id: &uuid::Uuid, email: &str) -> Result<String, DataError> {
        // generate url friendly token
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.register($1, $2, $3)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        if let Err(e) = client.execute(
            &stmt, 
            &[
                &id,
                &pg::Email::new(&email),
                &token
            ]
        ).await {
            error!("unable to execute statement: {:?}", e);
            return Err(DataError::DatabaseError);
        } else {
            return Ok(token);
        }
    }

    /// retrieve registration details
    pub async fn get_registration_info(&self, token: &str) -> Result<RegistrationInfo, DataError> {
        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::ConfigurationError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from iam.register_get_info($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &token
            ]).await {
                Err(e) => {
                    error!("unable to execute statement: {:?}", e);
                    return Err(DataError::DatabaseError);
                }
                Ok(row) => {
                    debug!("row: {:?}", row);

                    let token: String = row.get(0);
                    let email: String = row.get(1);
                    let created: DateTime<Utc> = row.get(2);

                    return Ok(RegistrationInfo {
                        token: token,
                        email: email,
                        created: created
                    });
                }
            }
    }

    pub async fn complete_registration(
        &self, 
        token: &str,
        pw: &str
    ) -> Result<(), DataError> {
        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.register_complete($1, $2)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.execute(
            &stmt,
            &[
                &token,
                &pw
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }


    pub async fn user_authenticate(
        &self,
        email: &str,
        pw: &str
    ) -> Result<bool, DataError> {
        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from iam.user_authenticate($1, $2)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &pg::Email::new(&email),
                &pw
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(row) => {
                debug!("row: {:?}", row);
                let authentic: bool = row.get(0);
                return Ok(authentic);
            }
        }
    }
}