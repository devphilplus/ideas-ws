use deadpool_postgres::Pool;
use log::{
    info,
    debug,
    error
};

use serde::{
    Serialize,
    Deserialize
};

use configuration::ApplicationConfiguration;
use common::{
    tenant::Tenant,
    user::User
};


pub struct Members {
    pool: Pool
}