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
use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;
use users::users::Users;

// use configuration::ApplicationConfiguration;
use crate::endpoints::{
    ApiResponse,
    default_options,
    default_service
};
use crate::classes::user::CurrentUser;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/join")
                .route(web::get().to(client_join_get))
                .route(web::post().to(client_join_post))
                .default_service(web::to(default_service))
        )
    ;
}

async fn client_join_get() -> impl Responder {
    info!("client_join_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn client_join_post(
    user: CurrentUser
) -> impl Responder {
    info!("client_join_post()");
    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}