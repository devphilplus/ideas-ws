use log::{
    info,
    debug,
    error
};

use configuration::ApplicationConfiguration;

#[derive(Debug)]
pub enum CountriesError {
    ToBeImplemented(String),
    ConfigurationError,
    ValidationError
}

#[derive(Clone)]
pub struct Countries {
    data: crate::data::countries::Data
}

impl Countries {

    pub fn new(
        cfg: ApplicationConfiguration,
        data: data::Data
    ) -> Self {
        // if let Ok(data) = crate::data::countries::Data::new(&cfg) {
        //     return Ok(Self {
        //         data: data
        //     });
        // }

        // return Err(CountriesError::ConfigurationError);
        let countries_data = crate::data::countries::Data::new(data);
        return Self {
            data: countries_data
        };
    }

    /// retrieve list of countries
    pub async fn countries(&self) -> Result<Vec<common::country::Country>, CountriesError> {
        info!("Countries::countries()");

        match self.data.fetch_all().await {
            Err(e) => {
                error!("unable to fetch countries");
                return Err(CountriesError::ToBeImplemented(String::from("countries()")));
            }
            Ok(countries) => {
                return Ok(countries);
            }
        }
    }
}
