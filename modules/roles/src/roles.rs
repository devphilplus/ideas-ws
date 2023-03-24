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
        cfg: ApplicationConfiguration
    ) -> Result<Self, RolesError> {
        if let Ok(data) = Data::new(&cfg) {
            return Ok(Self {
                cfg: cfg,
                data: data
            });
        }

        return Err(RolesError::ConfigurationError);
    }
}