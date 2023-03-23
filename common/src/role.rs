use log::{
    info,
    debug,
    error
};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    id: uuid::Uuid,
    active: bool,
    name: String,
    slug: String,
    description: String
}

impl Role {
    pub fn new(
        id: &uuid::Uuid,
        active: &bool,
        name: &str,
        slug: &str,
        description: &str
    ) {
        return Self {
            id: id.clone(),
            active: active.clone(),
            name: String::from(name),
            slug: String::from(slug),
            description: String::from(description)
        }
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

    pub fn slug(&self) -> String {
        return self.slug.clone();
    }

    pub fn description(&self) -> String {
        return self.description.clone();
    }
}