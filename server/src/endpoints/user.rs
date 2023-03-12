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
use crate::classes::user::CurrentUser;
use crate::classes::guards::permission::Permission;

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
                    .guard(Permission::new("permission.test"))
                    .to(user_set_password_post)
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
            debug!("result: {:?}", result);
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully retrieved user",
                    Some(json!({
                        "user": {
                            "email": user.email(),
                            "given_name": "result.given_name()",
                            "middle_name": "result.middle_name()",
                            "family_name": "result.family_name()"
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



    return HttpResponse::Ok()
        .json(ApiResponse::new(
            false,
            "unable to set user password",
            None
        ));
}