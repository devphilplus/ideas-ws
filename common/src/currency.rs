use log::{
    info,
    debug,
    error
};
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct Currency {
    id: u8,
    name: String,
    unit: String,
    symbol: String
}


impl Currency {

    pub fn new(
        id: &u8,
        name: &str,
        unit: &str,
        symbol: &str
    ) -> Self {
        return Self {
            id: id.clone(),
            name: String::from(name),
            unit: String::from(unit),
            symbol: String::from(symbol)
        };
    }

    pub fn id(&self) -> u8 {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn unit(&self) -> String {
        return self.unit.clone();
    }

    pub fn symbol(&self) -> String {
        return self.symbol.clone();
    }
}