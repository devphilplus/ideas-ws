use log::{
    info,
    debug,
    error
};

use actix_web::guard::{
    Guard,
    GuardContext
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
        if let Some(user) = context.req_data().get::<CurrentUser>() {
            debug!("Authenticated::check() (user): {:?}", user);
            return user.is_authenticated();
        }
        return false;
    }
}