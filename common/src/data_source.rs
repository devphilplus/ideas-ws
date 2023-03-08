use std::fmt;
use serde::de::value;
use serde_json::Value;

use configuration::ApplicationConfiguration;


#[derive(Debug, Clone)]
pub struct DataError {
    message: String
}

impl DataError {

    pub fn new(message: String) -> Self {
        return Self {
            message: message
        };
    }

    pub fn message(&self) -> String {
        return self.message.clone();
    }
}

impl fmt::Display for DataError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "DataError: {}", self.message);
    }
}


pub trait DataSource {
    
    fn get(&self, key: &str, params: &Value) -> Result<Value, DataError>;

    fn put(&self, key: &str, params: &Value) -> Result<Value, DataError>;
}