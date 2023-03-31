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

use tenants::tenants::Tenants;



#[derive(Debug, Serialize, Deserialize)]
struct TenantUserAddRequest {
    pub tenant_id: uuid::Uuid,
    pub name: String,
    pub slug: String,
    pub description: String
}


#[derive(Debug, Serialize, Deserialize)]
struct TenantGetInfoRequest {
    pub tenant_id: uuid::Uuid
}


#[derive(Debug, Serialize, Deserialize)]
struct TenantSetActiveRequest {
    pub tenant_id: uuid::Uuid,
    pub active: bool
}


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
        .service(
            web::resource("/get")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(tenant_get_get))
                .route(web::post()
                    .guard(Permission::new("permission.test"))
                    .to(tenant_get_post)
                )
                .default_service(web::to(default_service))
        )
        .service(
            web::resource("/set/active")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(tenant_set_active_get))
                .route(web::post()
                    .guard(Permission::new("permission.test"))
                    .to(tenant_set_active_post)
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
    user: CurrentUser,
    tenants: web::Data<Tenants>,
    params: web::Json<TenantUserAddRequest>
) -> impl Responder {
    info!("tenant_add_post");

    match tenants.tenant_add(
        params.id,
        &params.name,
        &params.slug,
        &params.description
    ).await {
        Err(e) => {
            error!("tenant_add_post(): {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    &"an error occured while trying to add a tenant",
                    None
                ));
        }
        Ok(_) => {
            return HttpResponse::Created()
                .json(ApiResponse::new(
                    true,
                    &"successfully added tenant",
                    None
                ));
        }
    }
}


async fn tenant_get_get() -> impl Responder {
    info!("tenant_get_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn tenant_get_post(
    user: CurrentUser,
    params: web::Json<TenantGetInfoRequest>
) -> impl Responder {
    info!("tenant_get_post()");
    debug!("params: {:?}", params);

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            "Service is up. version: 1.0.0.0.dev",
            None
        ));
}


async fn tenant_set_active_get() -> impl Responder {
    info!("tenant_set_active_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn tenant_set_active_post(
    user: CurrentUser,
    params: web::Json<TenantSetActiveRequest>
) -> impl Responder {
    info!("tenant_set_active_post()");
    debug!("params: {:?}", params);

    return HttpResponse::InternalServerError()
        .json(ApiResponse::new(
            false,
            "Service is up. version: 1.0.0.0.dev",
            None
        ));
}