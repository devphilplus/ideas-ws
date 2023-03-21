use log::{
    info,
    debug,
    error
};

use actix_web::{
    guard::{
        Guard,
        GuardContext    
    }
};

use crate::classes::user::CurrentUser;



#[derive(Debug)]
pub struct Authenticated {
}

impl Authenticated {
    pub fn new() -> Self {
        return Self {};
    }
}


impl Guard for Authenticated {

    fn check(&self, context: &GuardContext<'_>) -> bool {
        let data = context.req_data();
        if let Some(user) = data.get::<CurrentUser>() {
            debug!("current user: {:?}", user);
            return user.is_authenticated();
        }
        error!("Authenticated::check() returns false");
        return false;
    }
}