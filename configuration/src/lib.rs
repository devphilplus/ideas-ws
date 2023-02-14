use log::{
    info,
    debug,
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
pub struct ApplicationConfiguration {

    #[serde(rename = "bindHost")]
    pub bind_host: String,

    #[serde(rename = "bindPort")]
    pub bind_port: i32
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

    #[test]
    fn test_configuration() {
        env_logger::init();

        match ApplicationConfiguration::get() {
            None => {
                error!("configuration file not found");
                assert!(false);
            }
            Some(config) => {
                debug!("configuration loaded: {:?}", config);
                assert!(true);
            }
        }
    }
}
