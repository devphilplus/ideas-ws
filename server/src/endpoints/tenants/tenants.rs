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

use tenants::tenants::{Tenants, TenantsError};



#[derive(Debug, Serialize, Deserialize)]
struct TenantAddRequest {
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
struct TenantGetInfoBySlugRequest {
    pub name: String
}


#[derive(Debug, Serialize, Deserialize)]
struct TenantSetActiveRequest {
    pub tenant_id: uuid::Uuid,
    pub active: bool
}


#[derive(Debug, Serialize, Deserialize)]
struct TenantMembersRequest {
    pub tenant_slug: String
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
            web::resource("/fetch")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(tenants_fetch_get))
                .route(web::post()
                    .guard(Permission::new("permission.test"))
                    .to(tenants_fetch_post)
                )
                .default_service(web::to(default_service))
        )
        .service(
            web::resource("/update")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(tenant_update_get))
                .route(web::post()
                    .guard(Permission::new("permission.test"))
                    .to(tenant_update_post)
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
            web::resource("/get/slug")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(tenant_get_slug_get))
                .route(web::post()
                    .guard(Permission::new("permission.test"))
                    .to(tenant_get_slug_post)
                )
                .default_service(web::to(default_service))
        )
        .service(
            web::resource("/members/fetch")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(tenant_members_fetch_get))
                .route(web::post()
                    .guard(Permission::new("permission.test"))
                    .to(tenant_members_fetch_post)
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
    params: web::Json<TenantAddRequest>
) -> impl Responder {
    info!("tenant_add_post");

    match tenants.tenant_add(
        params.tenant_id,
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


async fn tenants_fetch_get() -> impl Responder {
    info!("tenants_fetch_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn tenants_fetch_post(
    user: CurrentUser,
    tenants: web::Data<Tenants>
) -> impl Responder {
    info!("tenants_fetch_post");

    match tenants.tenants().await {
        Err(e) => {
            error!("tenants_fetch_post(): {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    &"an error occured while trying to fetch tenants",
                    None
                ));
        }
        Ok(tenants) => {
            return HttpResponse::Created()
                .json(ApiResponse::new(
                    true,
                    &"successfully retrieved tenants",
                    Some(json!({
                        "tenants": tenants
                    }))
                ));
        }
    }
}


async fn tenant_update_get() -> impl Responder {
    info!("tenant_update_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn tenant_update_post(
    user: CurrentUser,
    tenants: web::Data<Tenants>,
    params: web::Json<TenantAddRequest>
) -> impl Responder {
    info!("tenant_update_post");

    match tenants.tenant_update(
        params.tenant_id,
        &params.name,
        &params.slug.as_str(),
        &params.description
    ).await {
        Err(e) => {
            error!("tenant_add_post(): {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    &"an error occured while trying to update tenant",
                    None
                ));
        }
        Ok(_) => {
            return HttpResponse::Created()
                .json(ApiResponse::new(
                    true,
                    &"successfully updated tenant",
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
    tenants: web::Data<Tenants>,
    params: web::Json<TenantGetInfoRequest>
) -> impl Responder {
    info!("tenant_get_post()");
    debug!("params: {:?}", params);

    match tenants.tenant_by_id(&params.tenant_id).await {
        Err(e) => {
            error!("tenant_get_post: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "//TODO an error occured while trying to retrieve tenant by id",
                    None
                ));
        }
        Ok(tenant) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved tenant by id",
                    Some(json!({
                        "tenant": tenant
                    }))
                ));
        }
    }
}


async fn tenant_get_slug_get() -> impl Responder {
    info!("tenant_get_slug_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn tenant_get_slug_post(
    user: CurrentUser,
    tenants: web::Data<Tenants>,
    params: web::Json<TenantGetInfoBySlugRequest>
) -> impl Responder {
    info!("tenant_get_slug_post()");
    debug!("params: {:?}", params);

    match tenants.tenant_by_name(&params.name).await {
        Err(e) => {
            error!("tenant_get_slug_post: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "//TODO an error occured while trying to retrieve tenant by slug",
                    None
                ));
        }
        Ok(tenant) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved tenant by slug",
                    Some(json!({
                        "tenant": tenant
                    }))
                ));
        }
    }
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


async fn tenant_members_fetch_get() -> impl Responder {
    info!("tenant_members_fetch_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn tenant_members_fetch_post(
    user: CurrentUser,
    tenants: web::Data<Tenants>,
    params: web::Json<TenantMembersRequest>
) -> impl Responder {
    info!("tenant_members_fetch_post()");
    debug!("params: {:?}", params);
    
    match tenants.tenant_by_slug(&params.tenant_slug).await {
        Err(e) => {
            error!("unable to fetch tenant members");
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "an error occured while trying to retrieve tenant members",
                    None
                ));
        }
        Ok(tenant) => {
            match tenants.tenant_users_fetch(&tenant.id()).await {
                Err(e) => {
                    error!("unable to fetch tenant members");
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse::new(
                            false,
                            "an error occured while trying to retrieve tenant members",
                            None
                        ));

                }
                Ok(members) => {
                    debug!("members: {:?}", members);
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            "successfully retrieved tenant members",
                            None
                        ))
                }
            }
        }
    }
}

