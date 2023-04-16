use log::{
    info,
    debug,
    error
};

use configuration::ApplicationConfiguration;

use crate::HrError;

// #[derive(Debug)]
// pub enum EmployeesError {
//     ToBeImplemented(String),
//     ConfigurationError,
//     MailerError,
//     ValidationError
// }

#[derive(Clone)]
pub struct Employees {
    data: crate::data::employees::Data,
    people: people::people::People
}

impl Employees {

    pub fn new(
        cfg: ApplicationConfiguration,
        people: people::people::People
    ) -> Result<Self, HrError> {
        if let Ok(data) = crate::data::employees::Data::new(&cfg) {
            return Ok(Self {
                data: data,
                people: people
            });
        }

        return Err(HrError::ConfigurationError);
    }

    pub fn add(
        &self,
        tenant_id: &uuid::Uuid,
        given_name: &str,
        middle_name: &str,
        family_name: &str,
        prefix: &str,
        suffix: &str,
        gender_id: &i32,
        ethnicity_id: &i32,
        marital_state_id: &i32
    ) -> Result<(), HrError> {
        return Err(HrError::ToBeImplemented(String::from("add")));
    }

    pub fn add_by_people_id(
        &self,
        tenant_id: &uuid::Uuid,
        people_id: &uuid::Uuid
    ) -> Result<(), HrError> {
        return Err(HrError::ToBeImplemented(String::from("add_by_people_id")));
    }
}
