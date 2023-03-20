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
    RecyclingMethod,
    Client 
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

use common::user::User;


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

    pub async fn by_email(
        &self,
        email: &str
    ) -> Result<User ,DataError> {
        info!("Data::by_email()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from iam.user_by_email($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &pg::Email::new(&email)
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
                let given_name: String = row.get(1);
                let middle_name: String = row.get(2);
                let family_name: String = row.get(3);
                // debug!("{:?} {:?} {:?}", given_name, middle_name, family_name);

                return Ok(User::new(
                    &id,
                    &active,
                    &email,
                    &given_name,
                    &middle_name,
                    &family_name
                ));
            }
        }
    }


    pub async fn user_set_active(
        &self,
        user_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DataError> {
        info!("Data::user_set_active()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.user_active($1, $2)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        if let Err(e) = client.execute(
            &stmt, 
            &[
                &user_id,
                &active
            ]
        ).await {
            error!("unable to execute statement: {:?}", e);
            return Err(DataError::DatabaseError);
        } else {
            return Ok(());
        }
    }

    pub async fn user_set_password(
        &self,
        user_id: &uuid::Uuid,
        password: &str
    ) -> Result<(), DataError> {
        info!("Data::user_set_password()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.user_set_pw($1, $2)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        if let Err(e) = client.execute(
            &stmt, 
            &[
                &user_id,
                &password
            ]
        ).await {
            error!("unable to execute statement: {:?}", e);
            return Err(DataError::DatabaseError);
        } else {
            return Ok(());
        }
    }

    pub async fn user_join_client(
        &self,
        user_id: &uuid::Uuid,
        client_id: &uuid::Uuid
    ) -> Result<(), DataError> {
        info!("user_join_client");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.user_client_add($1, $2)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        if let Err(e) = client.execute(
            &stmt, 
            &[
                &user_id,
                &client_id
            ]
        ).await {
            error!("unable to execute statement: {:?}", e);
            return Err(DataError::DatabaseError);
        } else {
            return Ok(());
        }
    }
}