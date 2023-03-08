use log::{
    info,
    debug,
    error
};

use std::str::FromStr;
use std::vec::Vec;

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
use crate::DataError;



pub struct Country {
    id: i32,
    name: String
}


#[derive(Debug, Clone)]
pub struct Countries {
    pool: Pool
}


impl Countries {

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

    pub async fn get_countries(&self) -> Result<(), DataError> {
        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from common.countries_fetch()"
        ).await;

        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query(
            &stmt,
            &[]
        ).await {
            Err(e) => {
                error!("unable to execute query: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(rows) => {
                debug!("result: {:?}", rows);
                return Ok(());
            }
        }

        // if let Err(e) = client.query(
        //     &stmt, 
        //     &[
        //         &id,
        //         &pg::Email::new(&email),
        //         &token
        //     ]
        // ).await {
        //     error!("unable to execute statement: {:?}", e);
        //     return Err(DataError::DatabaseError);
        // } else {
        //     return Ok(token);
        // }
    }
}