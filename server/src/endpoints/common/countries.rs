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

use crate::endpoints::{
    ApiResponse
};



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/countries")
                .route(web::get().to(countries_get))
                .route(web::post().to(countries_post))
        )
    ;
}


async fn countries_get() -> impl Responder {
    info!("countries_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn countries_post() -> impl Responder {
    info!("countries_post()");
    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}