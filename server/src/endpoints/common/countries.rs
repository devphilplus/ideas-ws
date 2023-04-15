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
                .route(web::get().to(countries_get))
                .route(web::post().to(countries_post))
        )
    ;
}


async fn countries_get() -> impl Responder {
    info!("countries_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn countries_post(
    countries: web::Data<util::countries::Countries>
) -> impl Responder {
    info!("countries_post()");

    match countries.countries().await {
        Err(e) => {
            error!("unable to fetch countries: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "an error occured while trying to fetch countries",
                    None
                ));
        }
        Ok(countries) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved countries",
                    Some(json!({
                        "countries": countries
                    }))
                ));
        }
    }
}