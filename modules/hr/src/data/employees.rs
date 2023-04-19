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

use common::hr::{
    people::People,
    employee::Employee
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

    /// add an employee
    pub async fn add(
        &self,
        tenant_id: &uuid::Uuid,
        employee_id: &uuid::Uuid,
        people_id: &uuid::Uuid
    ) -> Result<(), DataError> {
        info!("Data::add()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call hr.employee_add($1,$2,$3)"
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
                &employee_id,
                &people_id
            ]
        ).await {
            Err(e) => {
                error!("unable to employee record: {:?}", e);
                return Err(DataError::ToBeImplemented(String::from("Data::add()")));
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }

    /// fetch employees
    pub async fn fetch(
        &self,
        tenant_id: &uuid::Uuid
    ) -> Result<Vec<common::hr::employee::Employee>, DataError> {
        info!("Data::fetch()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from hr.employee_fetch($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query(
            &stmt,
            &[
                &tenant_id
            ]
        ).await {
            Err(e) => {
                error!("unable to retrieve employee records: {:?}", e);
                return Err(DataError::ToBeImplemented(String::from("Data::fetch()")));
            }
            Ok(rows) => {
                let result = rows.iter().map(|r| {
                    let employee_id: uuid::Uuid = r.get("id");

                    let people_id: uuid::Uuid = r.get("people_id");
                    let given_name: String = r.get("given_name");
                    let middle_name: String = r.get("middle_name");
                    let family_name: String = r.get("family_name");
                    let prefix: String = r.get("prefix");
                    let suffix: String = r.get("suffix");
                    let gender_id: i16 = r.get("gender_id");
                    let ethnicity_id: i16 = r.get("ethnicity_id");
                    let marital_state_id: i16 = r.get("marital_state_id");

                    return Employee::new(
                        &employee_id,
                        &People::new(
                            &people_id,
                            &true,
                            &given_name,
                            &middle_name,
                            &family_name,
                            &prefix,
                            &suffix,
                            &gender_id, 
                            &ethnicity_id,
                            &marital_state_id
                        )
                    );
                })
                .collect();

                return Ok(result);
            }
        }
    }
}