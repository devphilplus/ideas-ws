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



#[derive(Debug, Serialize, Deserialize)]
struct AccountAddRequest {
    pub account_id: String,
    pub name: String,
    pub description: String,
    pub currency_id: i16
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::get().to(account_add_get))
                .route(web::post().to(account_add_post))
        )
    ;
}


async fn account_add_get() -> impl Responder {
    info!("account_add_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn account_add_post(
    user: CurrentUser,
    params: web::Json<AccountAddRequest>
) -> impl Responder {
    info!("account_add_post()");
    debug!("params: {:?}", params);

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}