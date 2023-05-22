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


#[derive(Serialize, Deserialize, Debug)]
struct OrganizationRequest {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::get().to(organization_add_get))
                .route(web::post().to(organization_add_post))
        )
    ;
}



async fn organization_add_get() -> impl Responder {
    info!("organization_add_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn organization_add_post(
    user: CurrentUser,
    params: web::Json<OrganizationRequest>
) -> impl Responder {
    info!("organization_add_post()");
    debug!("params: {:?}", params);

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}