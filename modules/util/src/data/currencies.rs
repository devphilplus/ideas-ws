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
    NoTls
};
use tokio_postgres::config::{ Config };

use data::pg::DataError;


use configuration::{
    ApplicationConfiguration,
    ProviderType
};



#[derive(Debug, Clone)]
pub struct Data {
    pool: Pool
}

impl Data {

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

    pub fn new(
        data: data::Data
    ) -> Self {
        return Self {
            pool: data.get_pg_pool().unwrap()
        };
    }

    pub async fn fetch_all(&self) -> Result<Vec<common::currency::Currency>, DataError> {
        info!("Data::fetch_all()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from common.currencies_fetch()"
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
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(rows) => {
                // debug!("rows: {:?}", rows);

                let result = rows.iter().map(|r| {
                    let id: i32 = r.get("id");
                    let name: String = r.get("name");
                    let unit: String = r.get("unit");
                    let symbol: Option<String> = r.get("symbol");

                    return common::currency::Currency::new(
                        &id,
                        &name,
                        &unit,
                        symbol
                    );
                }).collect();

                return Ok(result);
            }
        }
    }
}