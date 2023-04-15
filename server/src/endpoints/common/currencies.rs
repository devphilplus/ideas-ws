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
use serde_json::json;

use crate::endpoints::{
    ApiResponse
};



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(currencies_get))
                .route(web::post().to(currencies_post))
        )
    ;
}


async fn currencies_get() -> impl Responder {
    info!("currencies_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn currencies_post(
    currencies: web::Data<util::currencies::Currencies>
) -> impl Responder {
    info!("currencies_post()");

    match currencies.currencies().await {
        Err(e) => {
            error!("unable to fetch currencies: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "an error occured while trying to fetch currencies",
                    None
                ));
        }
        Ok(currencies) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved currencies",
                    Some(json!({
                        "currencies": currencies
                    }))
                ));
        }
    }
}