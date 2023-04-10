use log::{
    info,
    debug,
    error
};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tenant {
    id: uuid::Uuid,
    active: bool,
    name: String
}


impl Tenant {

    pub fn new(
        id: &uuid::Uuid,
        active: &bool,
        name: &str
    ) -> Self {
        return Self {
            id: id.clone(),
            active: active.clone(),
            name: String::from(name)
        };
    }

    pub fn id(&self) -> uuid::Uuid {
        return self.id.clone();
    }

    pub fn active(&self) -> bool {
        return self.active.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}