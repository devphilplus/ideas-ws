pub mod pg;

use log::{
    info,
    debug,
    error
};
use std::str::FromStr;

use configuration::{
    ApplicationConfiguration,
    ProviderType
};

// use deadpool::managed::Object;
use deadpool_postgres::{ 
    Manager, 
    ManagerConfig, 
    Pool, 
    RecyclingMethod
};

use tokio_postgres::{
    NoTls,
    config::Config
};

// use pg::DataError;

#[derive(Debug, Clone)]
pub struct Data {
    pg_pool: Option<deadpool_postgres::Pool>
}

impl Data {

    pub fn new(
        cfg: ApplicationConfiguration
    ) -> Self {

        let mut data = Self {
            pg_pool: None
        };

        for p in &cfg.providers {
            match (p.provider_type) {
                ProviderType::Postgres => {
                    for url in &p.url {
                        match Config::from_str(&url) {
                            Err(e) => {
                                error!("error: {:?}", e);
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
                                            error!("error: {:?}", e);
                                        }
                                        Ok(pool) => {
                                            data.set_pg_pool(pool);
                                        }
                                    }
                            }
                        }
                    }
                }
                _ => {
                    error!("unknown provider type: {:?}", p.provider_type);
                }
            }
        }

        return data;
    }

    fn set_pg_pool(
        &mut self,
        pg_pool: deadpool_postgres::Pool
    ) {
        self.pg_pool = Some(pg_pool);
    }

    pub fn get_pg_pool(
        &self
    ) -> Option<deadpool_postgres::Pool> {
        return self.pg_pool.clone();
    }
}



#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
