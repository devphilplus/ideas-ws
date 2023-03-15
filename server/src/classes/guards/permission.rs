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
pub struct Permission {
    permission: String
}

impl Permission {
    pub fn new(permission: &str) -> Self {
        return Self {
            permission: String::from(permission)
        };
    }

    pub fn permission(&self) -> String {
        return self.permission.clone();
    }
}


impl Guard for Permission {

    fn check(&self, context: &GuardContext<'_>) -> bool {
        // debug!("Permission::check() (context): {:?}", context);
        if let Some(user) = context.req_data().get::<CurrentUser>() {
            debug!("//TODO Permission::check() (user): {:?}", user);
        }
        debug!("//TODO Permission::check()");
        return true;
    }
}