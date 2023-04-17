use log::{
    info,
    debug,
    error
};

use configuration::ApplicationConfiguration;

#[derive(Debug)]
pub enum PeopleError {
    ToBeImplemented(String),
    ConfigurationError,
    ValidationError
}

#[derive(Clone)]
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

    pub async fn add(
        &self,
        tenant_id: &uuid::Uuid,
        people_id: &uuid::Uuid,
        given_name: &str,
        middle_name: &str,
        family_name: &str,
        prefix: &str,
        suffix: &str,
        gender_id: &i16,
        ethnicity_id: &i16,
        marital_state_id: &i16
    ) -> Result<(), PeopleError> {
        info!("People::add()");

        match self.data.add(
            &tenant_id,
            &people_id,
            &given_name,
            &middle_name,
            &family_name,
            &prefix,
            &suffix,
            &gender_id,
            &ethnicity_id,
            &marital_state_id
        ).await {
            Err(e) => {
                error!("unable to add people record");
                return Err(PeopleError::ToBeImplemented(String::from("People::add")));
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }
}