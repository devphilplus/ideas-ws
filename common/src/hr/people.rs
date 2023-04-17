use log::{
    info,
    debug,
    error
};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct People {
    id: uuid::Uuid,
    active: bool,
    given_name: String,
    middle_name: String,
    family_name: String,
    prefix: String,
    suffix: String,
    gender_id: i16,
    ethnicity_id: i16,
    marital_state_id: i16
}

impl People {

    pub fn new(
        id: &uuid::Uuid,
        active: &bool,
        given_name: &str,
        middle_name: &str,
        family_name: &str,
        prefix: &str,
        suffix: &str,
        gender_id: &i16,
        ethnicity_id: &i16,
        marital_state_id: &i16
    ) -> Self {
        return Self {
            id: id.clone(),
            active: active.clone(),
            given_name: String::from(given_name),
            middle_name: String::from(middle_name),
            family_name: String::from(family_name),
            prefix: String::from(prefix),
            suffix: String::from(suffix),
            gender_id: gender_id.clone(),
            ethnicity_id: ethnicity_id.clone(),
            marital_state_id: marital_state_id.clone()
        };
    }

    pub fn id(&self) -> uuid::Uuid {
        return self.id.clone();
    }

    pub fn active(&self) -> bool {
        return self.active.clone();
    }

    pub fn given_name(&self) -> String {
        return self.given_name.clone();
    }

    pub fn middle_name(&self) -> String {
        return self.middle_name.clone();
    }

    pub fn family_name(&self) -> String {
        return self.family_name.clone();
    }

    pub fn prefix(&self) -> String {
        return self.prefix.clone();
    }

    pub fn suffix(&self) -> String {
        return self.suffix.clone();
    }

    pub fn gender_id(&self) -> i16 {
        return self.gender_id.clone();
    }

    pub fn ethnicity_id(&self) -> i16 {
        return self.ethnicity_id.clone();
    }

    pub fn marital_state_id(&self) -> i16 {
        return self.marital_state_id.clone();
    }
}