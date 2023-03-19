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
use crate::classes::guards::{
    authenticated::Authenticated,
    permission::Permission
};
use clients::clients::{Clients, ClientsError};
use crate::classes::user::CurrentUser;


#[derive(Debug, Serialize, Deserialize)]
struct ClientGetByNameRequest {
    pub name: String
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/join")
                .route(web::get().to(client_join_get))
                .route(web::post().to(client_join_post))
                .default_service(web::to(default_service))
        )
        .service(
            web::resource("/client/get")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_get_get))
                .route(web::post()
                    // .guard(Permission::new("permission.test"))
                    // .guard(Authenticated::new())
                    .to(client_get_post)
                )
                .default_service(web::to(default_service))
        )
        .service(
            web::resource("/client/active")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_active_get))
                .route(web::post()
                    // .guard(Permission::new("permission.test"))
                    .guard(Authenticated::new())
                    .to(client_active_post)
                )
                .default_service(web::to(default_service))
        )
        .service(
            web::resource("/client/add")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(client_add_get))
                .route(web::post()
                    // .guard(Permission::new("permission.test"))
                    .guard(Authenticated::new())
                    .to(client_add_post)
                )
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


async fn client_get_get() -> impl Responder {
    info!("client_get_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn client_get_post(
    clients: web::Data<Clients>,
    // user: CurrentUser,
    params: web::Json<ClientGetByNameRequest>
) -> impl Responder {
    info!("client_get_post()");
    debug!("params: {:?}", params);

    let client_name = params.name.clone();
    match clients.client_by_name(&client_name).await {
        Err(e) => {
            error!("client_get_post: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "an error occured while trying to retrieve client by name",
                    None
                ));
        }
        Ok(result) => {
            debug!("//TODO client_get_post: {:?}", result);

            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    false,
                    "Service is up. version: 1.0.0.0.dev",
                    None
                ));
        }
    }
}


async fn client_active_get() -> impl Responder {
    info!("client_active_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn client_active_post(
    user: CurrentUser
) -> impl Responder {
    info!("client_active_post()");
    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}



async fn client_add_get() -> impl Responder {
    info!("client_add_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn client_add_post(
    user: CurrentUser
) -> impl Responder {
    info!("client_add_post()");
    return HttpResponse::Ok()
        .json(ApiResponse {
            success: false,
            message: String::from("Service is up. version: 1.0.0.0.dev"),
            data: None
        });
}