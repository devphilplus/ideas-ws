use log::{
    info,
    debug,
    error
};


#[derive(Debug)]
pub struct User {
    id: uuid::Uuid,
    email: String,
    given_name: String,
    middle_name: String,
    family_name: String
}


impl User {

    pub fn new(
        id: &uuid::Uuid,
        email: &str,
        given_name: &str,
        middle_name: &str,
        family_name: &str
    ) -> Self {
        return Self {
            id: id.clone(),
            email: String::from(email),
            given_name: String::from(given_name),
            middle_name: String::from(middle_name),
            family_name: String::from(family_name)
        };
    }

    pub fn anonymous() -> Self {
        return Self {
            id: uuid::Uuid::nil(),
            email: String::from(""),
            given_name: String::from(""),
            middle_name: String::from(""),
            family_name: String::from("")
        };
    }

    pub fn id(&self) -> uuid::Uuid {
        return self.id.clone();
    }

    pub fn email(&self) -> String {
        return self.email.to_owned();
    }

    pub fn given_name(&self) -> String {
        return self.given_name.to_owned()
    }

    pub fn middle_name(&self) -> String {
        return self.middle_name.to_owned();
    }

    pub fn family_name(&self) -> String {
        return self.family_name.to_owned();
    }
}