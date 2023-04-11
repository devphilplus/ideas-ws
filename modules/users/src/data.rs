use common::tenant::Tenant;
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

    pub async fn by_id(
        &self,
        id: &uuid::Uuid
    ) -> Result<User ,DataError> {
        info!("Data::by_id()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from iam.user_by_id($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &id
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
                let email: String = row.get("email");
                let given_name: String = row.get("given_name");
                let middle_name: String = row.get("middle_name");
                let family_name: String = row.get("family_name");
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
                &pg::email::Email::new(&email)
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
                let email: String = row.get("email");
                let given_name: String = row.get("given_name");
                let middle_name: String = row.get("middle_name");
                let family_name: String = row.get("family_name");
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

    pub async fn user_tenant_add(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid
    ) -> Result<(), DataError> {
        info!("user_tenant_add");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.user_tenant_add($1, $2)"
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
                &tenant_id
            ]
        ).await {
            error!("unable to execute statement: {:?}", e);
            return Err(DataError::DatabaseError);
        } else {
            return Ok(());
        }
    }

    pub async fn user_tenant_set_active(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DataError> {
        info!("user_tenant_set_active");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.user_tenant_set_active($1, $2, $3)"
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
                &tenant_id,
                &active
            ]
        ).await {
            error!("unable to execute statement: {:?}", e);
            return Err(DataError::DatabaseError);
        } else {
            return Ok(());
        }
    }

    // pub async fn user_tenant_default(
    //     &self,
    //     user_id: &uuid::Uuid
    // ) -> Result<(), DataError> {
    //     info!("user_tenant_default");

    //     let result = self.pool.get().await;
    //     if let Err(e) = result {
    //         error!("unable to retrieve database client: {:?}", e);
    //         return Err(DataError::DatabaseError);
    //     }
    //     let client = result.unwrap();

    //     let result = client.prepare_cached(
    //         "call iam.user_tenant_fetch_default($1)"
    //     ).await;
    //     if let Err(e) = result {
    //         error!("unable to prepare database statement: {:?}", e);
    //         return Err(DataError::DatabaseError);
    //     }
    //     let stmt = result.unwrap();

    //     if let Err(e) = client.query_one(
    //         &stmt, 
    //         &[
    //             &user_id
    //         ]
    //     ).await {
    //         error!("unable to execute statement: {:?}", e);
    //         return Err(DataError::DatabaseError);
    //     } else {
    //         return Ok(());
    //     }
    // }

    pub async fn user_tenant_set_default(
        &self,
        user_id: &uuid::Uuid,
        tenant_id: &uuid::Uuid
    ) -> Result<(), DataError> {
        info!("user_tenant_set_default");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.user_tenant_set_default($1, $2)"
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
                &tenant_id
            ]
        ).await {
            error!("unable to execute statement: {:?}", e);
            return Err(DataError::DatabaseError);
        } else {
            return Ok(());
        }
    }

    pub async fn user_tenants(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<Vec<Tenant>, DataError> {
        info!("user_tenants");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from iam.user_tenants_fetch($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query(
            &stmt,
            &[
                &user_id
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(rows) => {
                // debug!("rows: {:?}", rows);
                let tenants = rows.iter().map(|r| {
                    let tenant_id: uuid::Uuid = r.get("tenant_id");
                    let tenant_active: bool = r.get("active");
                    let tenant_name: String = r.get("name");
                    let tenant_slug: String = r.get("slug");

                    return Tenant::new(
                        &tenant_id,
                        &tenant_active,
                        &tenant_name,
                        &tenant_slug
                    );
                }).collect();
                return Ok(tenants);
            }
        }
    }

    pub async fn user_tenants_default(
        &self,
        user_id: &uuid::Uuid
    ) -> Result<Tenant, DataError> {
        info!("user_tenants_default");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from iam.user_tenant_fetch_default($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &user_id
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(row) => {
                debug!("rows: {:?}", row);

                let tenant_id: uuid::Uuid = row.get("tenant_id");
                let tenant_active: bool = row.get("active");
                let tenant_name: String = row.get("name");
                let tenant_slug: String = row.get("slug");

                let tenant = Tenant::new(
                    &tenant_id,
                    &tenant_active,
                    &tenant_name,
                    &tenant_slug
                );
                return Ok(tenant);
            }
        }
    }
}