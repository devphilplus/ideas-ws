use log::{
    info,
    debug,
    error
};

use configuration::ApplicationConfiguration;


pub enum EmployeesError {
    ToBeImplemented(String),
    ConfigurationError,
    MailerError,
    ValidationError
}

pub struct Employees {
    data: crate::data::employees::Data
}

impl Employees {

    pub fn new(
        cfg: ApplicationConfiguration
    ) -> Result<Self, EmployeesError> {
        if let Ok(data) = crate::data::employees::Data::new(&cfg) {
            return Ok(Self {
                data: data
            });
        }

        return Err(EmployeesError::ConfigurationError);
    }

    pub fn add(
        &self,
        tenant_id: &uuid::Uuid,
        people_id: &uuid::Uuid
    ) -> Result<(), EmployeesError> {
        return Err(EmployeesError::ToBeImplemented(String::from("add")));
    }
}
