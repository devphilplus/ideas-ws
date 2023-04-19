use log::{
    info,
    debug,
    error
};
use serde::{Serialize, Deserialize};

use crate::hr::people::People;


#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    employee_id: uuid::Uuid,
    people: People
}


impl Employee {

    pub fn new(
        employee_id: &uuid::Uuid,
        people: &People
    ) -> Self {
        return Self {
            employee_id: employee_id.clone(),
            people: people.clone()
        }
    }


    pub fn employee_id(&self) -> uuid::Uuid {
        return self.employee_id.clone();
    }


    pub fn people(&self) -> People {
        return self.people.clone();
    }
}