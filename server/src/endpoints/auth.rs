use std::sync::Arc;
use std::str::FromStr;


use log::{
    info,
    debug,
    error
};

use actix_web::{
    HttpResponse, 
    Responder,
    web
};
use serde::{Serialize, Deserialize};

use configuration::ApplicationConfiguration;
use crate::endpoints::{
    ApiResponse
};

use auth::auth::Auth;

#[derive(Debug, Serialize, Deserialize)]
struct AuthRegistrationRequest {
    pub id: uuid::Uuid,
    pub email: String
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("register")
                .route(web::get().to(register_get))
                .route(web::post().to(register_post))
        )
        .service(
            web::resource("register/complete")
                .route(web::get().to(register_complete_get))
                .route(web::post().to(register_complete_post))
        )
    ;
}

async fn register_get() -> impl Responder {
    info!("register_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn register_post(
    auth: web::Data<Auth>,
    params: web::Json<AuthRegistrationRequest>
) -> impl Responder {
    info!("register_post()");

    if let Err(e) = auth.register(
        &params.id, 
        &params.email
    ) {
        return HttpResponse::InternalServerError()
            .json(ApiResponse::new(
                false,
                format!("{}", e),
                None
            ));
    }

    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}

async fn register_complete_get() -> impl Responder {
    info!("register_complete_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn register_complete_post() -> impl Responder {
    info!("register_complete_post()");
    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}
