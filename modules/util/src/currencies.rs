use log::{
    info,
    debug,
    error
};

use configuration::ApplicationConfiguration;

#[derive(Debug)]
pub enum CurrenciesError {
    ToBeImplemented(String),
    ConfigurationError,
    ValidationError
}

#[derive(Clone)]
pub struct Currencies {
    data: crate::data::currencies::Data
}

impl Currencies {

    pub fn new(
        cfg: ApplicationConfiguration,
        data: data::Data
    ) -> Self {
        // if let Ok(data) = crate::data::currencies::Data::new(&cfg) {
        //     return Ok(Self {
        //         data: data
        //     });
        // }

        // return Err(CurrenciesError::ConfigurationError);
        let currencies_data = crate::data::currencies::Data::new(data);
        return Self {
            data: currencies_data
        };
    }

    /// retrieve list of currencies
    pub async fn currencies(&self) -> Result<Vec<common::currency::Currency>, CurrenciesError> {
        info!("Currencies::currencies()");

        match self.data.fetch_all().await {
            Err(e) => {
                error!("unable to fetch currencies");
                return Err(CurrenciesError::ToBeImplemented(String::from("currencies()")));
            }
            Ok(currencies) => {
                return Ok(currencies);
            }
        }
    }
}
