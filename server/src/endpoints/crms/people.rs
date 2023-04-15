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

use crate::endpoints::{
    ApiResponse
};
use crate::classes::user::CurrentUser;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::get().to(people_add_get))
                .route(web::post().to(people_add_post))
        )
    ;
}



async fn people_add_get() -> impl Responder {
    info!("people_add_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn people_add_post(
    user: CurrentUser
) -> impl Responder {
    info!("people_add_post()");
    // debug!("params: {:?}", params);

    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}