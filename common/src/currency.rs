use log::{
    info,
    debug,
    error
};
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct Currency {
    id: i32,
    name: String,
    unit: String,
    symbol: Option<String>
}


impl Currency {

    pub fn new(
        id: &i32,
        name: &str,
        unit: &str,
        symbol: Option<String>
    ) -> Self {
        return Self {
            id: id.clone(),
            name: String::from(name),
            unit: String::from(unit),
            symbol: symbol
        };
    }

    pub fn id(&self) -> i32 {
        return self.id.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn unit(&self) -> String {
        return self.unit.clone();
    }

    pub fn symbol(&self) -> Option<String> {
        return self.symbol.clone();
    }
}