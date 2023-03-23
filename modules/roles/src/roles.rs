use configuration::ApplicationConfiguration;
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


#[derive(Debug, Clone)]
pub struct Roles {
    cfg: ApplicationConfiguration,
    data: crate::data::Data
}


impl Roles {
}