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

use crate::data::Data;


pub struct Members {
    data: Data
}


impl Members {

    pub fn new(
        data: &crate::data::Data
    ) -> Self {
        return Members {
            data: data.clone()
        };
    }

    pub fn tenant_join(
        &self,
        tenant_id: &uuid::Uuid,
        member_id: &uuid::Uuid
    ) {
        
    }
}