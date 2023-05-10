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
        cfg: ApplicationConfiguration,
        data: data::Data
    ) -> Self {
        let people_data = crate::data::people::Data::new(data);
        return Self {
            cfg: cfg,
            data: people_data
        };
    }

    pub fn add(
        &self,
        tenant_id: &uuid::Uuid,
        people_id: &uuid::Uuid
    ) -> Result<(), PeopleError> {
        return Err(PeopleError::ToBeImplemented(String::from("add")));
    }
}
