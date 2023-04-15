use log::{
    info,
    debug,
    error
};

use configuration::ApplicationConfiguration;


pub enum CountriesError {
    ToBeImplemented(String),
    ConfigurationError,
    MailerError,
    ValidationError
}

pub struct Countries {
    data: crate::data::countries::Data
}

impl Countries {

    pub fn new(
        cfg: ApplicationConfiguration
    ) -> Result<Self, CountriesError> {
        if let Ok(data) = crate::data::countries::Data::new(&cfg) {
            return Ok(Self {
                data: data
            });
        }

        return Err(CountriesError::ConfigurationError);
    }
}
