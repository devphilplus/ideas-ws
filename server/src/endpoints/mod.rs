pub mod status;
pub mod auth;
pub mod user;
pub mod common;
pub mod clients;
pub mod tenants;
pub mod accounting;
pub mod inventory;


use log::{
    info,
    debug
};



use serde::{
    Serialize,
    Deserialize
};
use serde_json::Value;

use actix_web::{
    HttpResponse, 
    Responder
};

// use crate::classes::extractors::user::User;
use crate::classes::user::CurrentUser;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    success: bool,
    message: String,
    data: Option<Value>
}


impl ApiResponse {

    pub fn new(
        success: bool,
        message: &str,
        data: Option<Value>
    ) -> Self {
        return Self {
            success: success,
            message: message.to_string(),
            data: data
        };
    }
}


pub async fn default_options() -> impl Responder {
    info!("endpoints::default_options()");
    return HttpResponse::Ok()
        .finish();
}

pub async fn default_service(
    user: CurrentUser
) -> impl Responder {
    info!("endpoints::default_service()");
    // debug!("user: {:?}", user);

    if user.is_authenticated() {
        return HttpResponse::Forbidden()
            .json(ApiResponse::new(
                false,
                "user does not have permissions",
                None
            ));
    } else {
        return HttpResponse::Unauthorized()
            .json(ApiResponse::new(
                false,
                "user is not authenticated",
                None
            ));
    }
}