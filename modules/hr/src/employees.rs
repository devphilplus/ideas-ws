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

    pub async fn add(
        &self,
        tenant_id: &uuid::Uuid,
        employee_id: &uuid::Uuid,
        people_id: &uuid::Uuid
    ) -> Result<(), HrError> {
        info!("Employees::add()");

        match self.data.add(
            &tenant_id,
            &employee_id,
            &people_id
        ).await {
            Err(e) => {
                error!("unable to add employee record: {:?}", e);
                return Err(HrError::ToBeImplemented(String::from("Data::add()")));
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }
}
