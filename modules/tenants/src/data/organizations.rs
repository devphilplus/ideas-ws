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

use data::pg::DataError;


#[derive(Debug, Clone)]
pub struct OrganizationsData {
    pool: Pool
}


impl OrganizationsData {

    // pub fn new(cfg: &ApplicationConfiguration) -> Result<Self, DataError> {
    //     for p in &cfg.providers {
    //         if matches!(p.provider_type, ProviderType::Postgres) {
    //             for url in &p.url {
    //                 match Config::from_str(&url) {
    //                     Err(e) => {
    //                         debug!("error: {:?}", e);
    //                         return Err(DataError::ConfigurationError);
    //                     }
    //                     Ok(c) => {
    //                         let mgr = Manager::from_config(
    //                             c, 
    //                             NoTls, 
    //                             ManagerConfig { recycling_method: RecyclingMethod::Fast }
    //                         );
    //                         match Pool::builder(mgr)
    //                             .max_size(4)
    //                             .build() {
    //                                 Err(e) => {
    //                                     debug!("error: {:?}", e);
    //                                     return Err(DataError::ToBeImplemented(String::from("new")));
    //                                 }
    //                                 Ok(pool) => {
    //                                     return Ok(Self {
    //                                         pool: pool
    //                                     });
    //                                 }
    //                             }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     return Err(DataError::ConfigurationError);
    // }

    pub fn new(data: data::Data) -> Self {
        return Self {
            pool: data.get_pg_pool().unwrap()
        };
    }

    /// add organization record
    pub async fn add_organization(
        &self,
        tenant_id: &uuid::Uuid,
        organization_id: &uuid::Uuid,
        name: &str,
        description: &str
    ) -> Result<(), DataError> {
        info!("OrganizationsData::add_organization()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call tenants.organization_add($1,$2,$3,$4)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.execute(
            &stmt,
            &[
                &tenant_id,
                &organization_id,
                &name,
                &description
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

    // set organization active status
    pub async fn organization_set_active(
        &self,
        organization_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DataError> {
        info!("OrganizationsData::organization_set_active()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call tenants.organization_set_active($1,$2)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.execute(
            &stmt,
            &[
                &organization_id,
                &active
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
}