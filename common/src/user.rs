use log::{
    info,
    debug,
    error
};


#[derive(Debug)]
pub struct User {
    email: String
}

impl User {
    pub fn new(
        email: &str
    ) -> Self {
        return Self {
            email: String::from(email)
        };
    }

    pub fn anonymous() -> Self {
        return Self {
            email: String::from("")
        };
    }

    pub fn email(&self) -> String {
        return self.email.to_owned();
    }
}