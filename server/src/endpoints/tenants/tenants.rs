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
    ApiResponse,
    default_options,
    default_service
};
use crate::classes::user::CurrentUser;
use crate::classes::guards::{
    authenticated::Authenticated,
    permission::Permission
};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(tenant_add_get))
                .route(web::post()
                    .guard(Permission::new("permission.test"))
                    .to(tenant_add_post)
                )
                .default_service(web::to(default_service))
        )
    ;
}


async fn tenant_add_get() -> impl Responder {
    info!("tenant_add_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn tenant_add_post(
    user: CurrentUser
) -> impl Responder {
    info!("tenant_add_post");
    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            &"todo",
            None
        ));
}