use log::{
    info,
    debug,
    error
};

use chrono::prelude::*;
use serde::{
    Serialize,
    Deserialize
};

use configuration::{
    ApplicationConfiguration,
    ProviderType
};
use crate::data::Data;


#[derive(Debug)]
pub enum RolesError {
    ToBeImplemented(String),
    ConfigurationError
}


#[derive(Debug, Clone)]
pub struct Roles {
    cfg: ApplicationConfiguration,
    data: crate::data::Data
}


impl Roles {

    pub fn new(
        cfg: ApplicationConfiguration,
        data: data::Data
    ) -> Self {
        let roles_data = crate::data::Data::new(data);
        return Self {
            cfg: cfg,
            data: roles_data
        };
    }
}