pub mod status;
pub mod auth;
pub mod common;
pub mod clients;
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