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

use configuration::ApplicationConfiguration;
use crate::endpoints::{
    ApiResponse,
    default_options
};
use crate::classes::user::User;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/current")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(current_get))
                .route(web::post().to(current_post))
        )
    ;
}


async fn current_get() -> impl Responder {
    info!("current_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn current_post(
    user: User
) -> impl Responder {
    info!("current_post()");

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            "an error occured while trying to register",
            None
        ));
}