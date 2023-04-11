use common::user::User;
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

use pg::DataError;
use common::tenant::Tenant;


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

    pub async fn tenant_by_id(
        &self,
        tenant_id: &uuid::Uuid
    ) -> Result<Tenant, DataError> {
        info!("Data::tenant_by_name()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from tenants.tenant_by_id($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &tenant_id
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(row) => {
                debug!("row: {:?}", row);

                let tenant_id: uuid::Uuid = row.get("id");
                let tenant_active: bool = row.get("active");
                let tenant_name: String = row.get("name");
                let tenant_slug: String = row.get("slug");

                return Ok(Tenant::new(
                    &tenant_id,
                    &tenant_active,
                    &tenant_name,
                    &tenant_slug
                ));
            }
        }
    }

    pub async fn tenant_by_name(
        &self,
        name: &str
    ) -> Result<Tenant, DataError> {
        info!("Data::tenant_by_name()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database client: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from tenants.tenant_by_name($1)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
                &name
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(row) => {
                debug!("row: {:?}", row);

                let tenant_id: uuid::Uuid = row.get("id");
                let tenant_active: bool = row.get("active");
                let tenant_name: String = row.get("name");
                let tenant_slug: String = row.get("slug");

                return Ok(Tenant::new(
                    &tenant_id,
                    &tenant_active,
                    &tenant_name,
                    &tenant_slug
                ));
            }
        }
    }


    pub async fn tenant_add(
        &self,
        id: &uuid::Uuid,
        name: &str,
        slug: &str,
        description: &str
    ) -> Result<(), DataError> {
        info!("Data::tenant_add()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database tenant: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call tenants.tenant_add($1, $2, $3, $4)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.execute(
            &stmt,
            &[
                &id,
                &name,
                &slug,
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


    pub async fn tenant_update(
        &self,
        id: &uuid::Uuid,
        name: &str,
        slug: &str,
        description: &str
    ) -> Result<(), DataError> {
        info!("Data::tenant_update()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database tenant: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call tenants.tenant_update($1, $2, $3, $4)"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.execute(
            &stmt,
            &[
                &id,
                &name,
                &pg::slug::Slug::new(&slug),
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


    pub async fn tenant_set_active(
        &self,
        tenant_id: &uuid::Uuid,
        active: &bool
    ) -> Result<(), DataError> {
        info!("Data::tenant_set_active()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database tenant: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call tenants.set_active($1, $2)"
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

    pub async fn tenant_users_fetch(
        &self,
        tenant_id: &uuid::Uuid
    ) -> Result<Vec<common::user::User>, DataError> {
        info!("Data::tenant_users_fetch()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database tenant: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "call iam.tenant_set_active($1)"
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
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(results) => {
                debug!("Data::tenant_users_fetch(): {:?}", results);
                let users = results.iter().map(|r| {
                    let id: uuid::Uuid = r.get("id");
                    let email: String = r.get("email");
                    let active: bool = r.get("active");

                    return User::new(
                        &id,
                        &active,
                        &email,
                        "",
                        "",
                    ""
                    );
                }).collect();
                return Ok(users);
            }
        }

    }

    pub async fn tenant_default_fetch(
        &self,
        tenant_id: &uuid::Uuid
    ) -> Result<common::tenant::Tenant, DataError> {
        info!("Data::tenant_users_fetch()");

        let result = self.pool.get().await;
        if let Err(e) = result {
            error!("unable to retrieve database tenant: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let client = result.unwrap();

        let result = client.prepare_cached(
            "select * from tenants.tenant_default_fetch()"
        ).await;
        if let Err(e) = result {
            error!("unable to prepare database statement: {:?}", e);
            return Err(DataError::DatabaseError);
        }
        let stmt = result.unwrap();

        match client.query_one(
            &stmt,
            &[
            ]
        ).await {
            Err(e) => {
                error!("unable to execute statement: {:?}", e);
                return Err(DataError::DatabaseError);
            }
            Ok(row) => {
                debug!("Data::tenant_default_fetch(): {:?}", row);

                let tenant_id: uuid::Uuid = row.get("id");
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