use log::{
    info,
    debug,
    error
};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Country {
    id: u32,
    name: String,
    alpha_2: String,
    alpha_3: String,
    currency_id: u32
}


impl Country {

    pub fn new(
        id: &u32,
        name: &str,
        alpha_2: &str,
        alpha_3: &str,
        currency_id: &u32
    ) -> Self {
        return Self {
            id: id.clone(),
            name: String::from(name),
            alpha_2: String::from(alpha_2),
            alpha_3: String::from(alpha_3),
            currency_id: currency_id.clone()
        };
    }

    pub fn id(&self) -> u32 {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn alpha_2(&self) -> String {
        return self.alpha_2.clone();
    }

    pub fn alpha_3(&self) -> String {
        return self.alpha_3.clone();
    }

    pub fn currency_id(&self) -> u32 {
        return self.currency_id.clone();
    }
}