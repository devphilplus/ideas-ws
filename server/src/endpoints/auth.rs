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
use mailer::Mailer;
use serde::{Serialize, Deserialize};

use configuration::ApplicationConfiguration;
use crate::endpoints::{
    ApiResponse
};

use auth::auth::{Auth, AuthError};

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
    // cfg: web::Data<ApplicationConfiguration>,
    auth: web::Data<Auth>,
    params: web::Json<AuthRegistrationRequest>
) -> impl Responder {
    info!("register_post()");
    debug!("parameters: {:?}", params);

    if let Err(e) = auth.register(
        &params.id, 
        &params.email
    ).await {
        error!("unable to register: {:?}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::new(
                false,
                format!("{}", e),
                None
            ));
    } else {
        return HttpResponse::Ok()
            .json(ApiResponse::new(
                true,
                String::from("success"),
                None
            ));
    }
}

async fn register_complete_get() -> impl Responder {
    info!("register_complete_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn register_complete_post(
    auth: web::Data<Auth>,
) -> impl Responder {
    info!("register_complete_post()");
    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}
