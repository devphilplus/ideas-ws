use log::{
    info,
    debug,
    error
};


use configuration::ApplicationConfiguration;


#[derive(Debug)]
pub enum DataError {
    ToBeImplemented(String)
}


pub struct Data {
    
}

impl Data {

    pub fn new() -> Self {
        return Self {

        };
    }

    pub fn register(&self, token: &str, email: &str) -> Result<(), DataError> {
        return Err(DataError::ToBeImplemented(String::from("register")));
    }
}