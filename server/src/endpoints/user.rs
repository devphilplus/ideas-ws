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
// use clients::clients::Clients;
use tenants::tenants::Tenants;

// use configuration::ApplicationConfiguration;
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

use auth::{
    // user::User,
    auth::{
    Auth,
    AuthError
}};


#[derive(Debug, Serialize, Deserialize)]
struct UserPasswordRequest {
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
struct UserTenantJoinRequest {
    pub tenant: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/current")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(current_get))
                .route(web::post()
                    .guard(Permission::new("permission.test"))
                    .to(current_post)
                )
                .default_service(web::to(default_service))
        )
        .service(
            web::resource("/password/set")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_set_password_get))
                .route(web::post()
                    // .guard(Permission::new("permission.test"))
                    .guard(Authenticated::new())
                    .to(user_set_password_post)
                )
                .default_service(web::to(default_service))
        )
        .service(
            web::resource("/tenant/join")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(user_tenant_join_get))
                .route(web::post()
                    // .guard(Permission::new("permission.test"))
                    .guard(Authenticated::new())
                    .to(user_tenant_join_post)
                )
                .default_service(web::to(default_service))
        )
    ;
}


async fn current_get() -> impl Responder {
    info!("current_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn current_post(
    // auth: web::Data<Auth>,
    users: web::Data<Users>,
    user: CurrentUser
) -> impl Responder {
    info!("current_post()");

    debug!("params: {:?}", user);
    let email = user.email().clone();

    match users.user_by_email(&email).await {
        Err(e) => {
            error!("unable to get user: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "unable to retrieve user",
                    None
                ));
        }
        Ok(result) => {
            // debug!("result: {:?}", result);
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved user",
                    Some(json!({
                        "user": {
                            "email": user.email(),
                            "given_name": result.given_name(),
                            "middle_name": result.middle_name(),
                            "family_name": result.family_name()
                        }
                    }))
                ))
        }
    }
}

async fn user_set_password_get() -> impl Responder {
    info!("user_set_password_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn user_set_password_post(
    users: web::Data<Users>,
    user: crate::classes::user::CurrentUser,
    params: web::Json<UserPasswordRequest>
) -> impl Responder {
    info!("user_set_password_post()");

    debug!("user: {:?}", user);
    debug!("params: {:?}", params);

    match users.user_set_password(
        &user.id(),
        &params.password
    ).await {
        Err(e) => {
            debug!("user_set_password_post: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "unable to change password",
                    None
                ));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully changed password",
                    None
                ));
        }
    }
}

async fn user_tenant_join_get() -> impl Responder {
    info!("user_client_join_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn user_tenant_join_post(
    // clients: web::Data<Clients>,
    tenants: web::Data<Tenants>,
    users: web::Data<Users>,
    user: crate::classes::user::CurrentUser,
    params: web::Json<UserTenantJoinRequest>
) -> impl Responder {
    info!("user_client_join_post()");
    debug!("user: {:?}", user);
    debug!("params: {:?}", params);
    
    match tenants.tenant_by_name(&params.tenant).await {
        Err(e) => {
            error!("unable to fetch client by name: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "an error occured while trying to join a tenant",
                    None
                ));
        }
        Ok(tenant) => {
            debug!("tenant found: {:?}", tenant);

            match users.user_join_client(
            &user.id(),
            &tenant.id()
            ).await {
                Err(e) => {
                    error!("user_tenant_join_post: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse::new(
                            false,
                            "an error occured while trying to join a tenant",
                            None
                        ));
                }
                Ok(result) => {
                    debug!("user_tenant_join_post: {:?}", result);
                    return HttpResponse::Ok()
                        .json(ApiResponse::new(
                            true,
                            "successfully joined tenant",
                            None
                        ));
                }
            }
        }
    }
}