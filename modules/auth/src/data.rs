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

    pub async fn register(&self, token: &uuid::Uuid, email: &str) -> Result<(), DataError> {
        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached("call iam.register($1, $2)").await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        if let Err(e) = client.execute(
            &stmt, 
            &[
                &token,
                &pg::Email::new(&email)
            ]
        ).await {
            error!("unable to execute statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        
        return Ok(());
    }
}