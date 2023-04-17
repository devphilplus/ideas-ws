use common::tenant;
use log::{
    info,
    debug,
    error
};
use std::str::FromStr;

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

use pg::DataError;


use configuration::{
    ApplicationConfiguration,
    ProviderType
};



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

    pub async fn add(
        &self,
        tenant_id: &uuid::Uuid,
        people_id: &uuid::Uuid,
        given_name: &str,
        middle_name: &str,
        family_name: &str,
        prefix: &str,
        suffix: &str,
        gender_id: &i16,
        ethnicity_id: &i16,
        marital_state_id: &i16
    ) -> Result<(), DataError> {
        info!("Data::add()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call people.people_add($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"
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
                &people_id,
                &given_name,
                &middle_name,
                &family_name,
                &prefix,
                &suffix,
                &gender_id,
                &ethnicity_id,
                &marital_state_id
            ]
        ).await {
            Err(e) => {
                error!("unable to add people record: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }

    pub async fn by_id(
        &self,
        people_id: &uuid::Uuid
    ) -> Result<common::hr::people::People, DataError> {
        info!("Data::by_id()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * people.people_get_by_id($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &people_id
            ]
        ).await {
            Err(e) => {
                error!("unable to retrieve people record by id: {:?}", e);
                return Err(DataError::ToBeImplemented(String::from("Data::by_id()")));
            }
            Ok(row) => {
                debug!("row: {:?}", row);

                
                return Ok(people);
            }
        }
    }
}