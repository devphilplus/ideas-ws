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
    RecyclingMethod 
};
use tokio_postgres::{
    NoTls,
    // row::Row
};
use tokio_postgres::config::{ Config };


use configuration::{
    ApplicationConfiguration,
    ProviderType
};


#[derive(Debug)]
pub enum DataError {
    ToBeImplemented(String)
}


pub struct Data {
    pool: Pool
}

impl Data {

    pub fn new(cfg: &ApplicationConfiguration) -> Result<Self, DataError> {
        for p in cfg.providers {
            if matches!(p.provider_type, ProviderType::Postgres) {
                for url in p.url {
                    match Config::from_str(&url) {
                        Err(e) => {
                            return Err(DataError::ToBeImplemented(String::from("unable to create database configuration from url")));
                        }
                        Ok(c) => {
                            let mgr = Manager::from_config(
                                c, 
                                NoTls, 
                                ManagerConfig { recycling_method: RecyclingMethod::Fast }
                            );
                        }
                    }
                }
            }
        }

        return Err(DataError::ToBeImplemented(String::from("new")));
    }

    pub fn register(&self, token: &uuid::Uuid, email: &str) -> Result<(), DataError> {
        return Err(DataError::ToBeImplemented(String::from("register")));
    }
}