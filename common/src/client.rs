use log::{
    info,
    debug,
    error
};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    id: uuid::Uuid,
    name: String
}


impl Client {

    pub fn new(
        id: &uuid::Uuid,
        name: &str
    ) -> Self {
        return Self {
            id: id.clone(),
            name: String::from(name)
        };
    }
}