use log::{
    info,
    debug,
    error
};

use configuration::ApplicationConfiguration;


pub enum PeopleError {
    ToBeImplemented(String),
    ConfigurationError,
    MailerError,
    ValidationError
}

pub struct People {
    data: crate::data::people::Data
}

impl People {

    pub fn new(
        cfg: ApplicationConfiguration
    ) -> Result<Self, PeopleError> {
        if let Ok(data) = crate::data::people::Data::new(&cfg) {
            return Ok(Self {
                data: data
            });
        }

        return Err(PeopleError::ConfigurationError);
    }

    pub fn add(
        &self,
        tenant_id: &uuid::Uuid,
        people_id: &uuid::Uuid
    ) -> Result<(), PeopleError> {
        return Err(PeopleError::ToBeImplemented(String::from("add")));
    }
}
