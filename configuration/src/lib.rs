use log::{
    // info,
    // debug,
    error
};

use std::{
    env,
    fs
};

use serde::{
    Serialize,
    Deserialize
};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProviderType {
    #[serde(rename = "postgres")]
    Postgres
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    pub url: Vec<String>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MailDefaults {
    pub from: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mail {
    pub host: String,
    pub user: String,
    pub password: String,

    //#[serde(rename = "defaults")]
    pub defaults: MailDefaults
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationConfiguration {

    #[serde(rename = "bindHost")]
    pub bind_host: String,

    #[serde(rename = "bindPort")]
    pub bind_port: i32,

    #[serde(rename = "baseURL")]
    pub base_url: String,

    pub providers: Vec<Provider>,
    pub mailer: Mail,
}


impl ApplicationConfiguration {
    pub fn get() -> Option<ApplicationConfiguration> {
        if let Ok(cfg) = env::var("CFG") {
            match fs::read_to_string(cfg) {
                Err(e) => {
                    error!("unable to read contents of file: {:?}", e);
                    return None;
                }
                Ok(contents) => {
                    let result: Result<ApplicationConfiguration, serde_json::Error> = serde_json::from_str(&contents);
                    match result {
                        Err(e) => {
                            error!("unable to parse contents of file: {:?}", e);
                            return None;
                        }
                        Ok(config) => {
                            return Some(config);
                        }
                    }
                }
            }
        } else {
            error!("missing environment variable 'CFG'");
            return None;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use log::{ debug };

    #[test]
    fn test_configuration() {
        env_logger::init();

        match ApplicationConfiguration::get() {
            None => {
                error!("configuration file not found");
                assert!(false, "configuration file not found");
            }
            Some(config) => {
                debug!("configuration loaded: {:?}", config);
                assert!(true);
            }
        }
    }
}
