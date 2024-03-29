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

use http::header::AUTHORIZATION;


use configuration::ApplicationConfiguration;
use crate::endpoints::{
    ApiResponse,
    default_options
};

use auth::auth::{Auth, AuthError};

#[derive(Debug, Serialize, Deserialize)]
struct AuthRegistrationRequest {
    pub id: uuid::Uuid,
    pub email: String
}


#[derive(Debug, Serialize, Deserialize)]
struct AuthRegistrationInfoRequest {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthRegistrationCompleteRequest {
    pub token: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SignInRequest {
    pub email: String,
    pub password: String
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/register")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(register_get))
                .route(web::post().to(register_post))
        ).service(
            web::resource("/register/info")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(register_info_get))
                .route(web::post().to(register_info_post))
        )
        .service(
            web::resource("/register/complete")
                .route(web::method(http::Method::OPTIONS).to(default_options))
                .route(web::get().to(register_complete_get))
                .route(web::post().to(register_complete_post))
        )
        .service(
            web::resource("/sign-in")
            .route(web::method(http::Method::OPTIONS).to(default_options))
            .route(web::get().to(auth_signin_get))
            .route(web::post().to(auth_signin_post))
        )
    ;
}

async fn register_get() -> impl Responder {
    info!("register_get()");
    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn register_post(
    // cfg: web::Data<ApplicationConfiguration>,
    auth: web::Data<Auth>,
    params: web::Json<AuthRegistrationRequest>
) -> impl Responder {
    info!("register_post()");

    if let Err(e) = auth.register(
        &params.id, 
        &params.email
    ).await {
        error!("unable to register: {:?}", e);
        return HttpResponse::InternalServerError()
            .json(ApiResponse::new(
                false,
                "an error occured while trying to register",
                None
            ));
    } else {
        return HttpResponse::Ok()
            .json(ApiResponse::new(
                true,
                "success",
                None
            ));
    }
}

async fn register_info_get() -> impl Responder {
    info!("register_info_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn register_info_post(
    auth: web::Data<Auth>,
    params: web::Json<AuthRegistrationInfoRequest>
) -> impl Responder {
    info!("register_info_post()");

    match auth.get_registration_info(&params.token).await {
        Err(e) => {
            error!("unable to get registration info: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "unable to retrieve registration info",
                    None
                ));
        }
        Ok(info) => {
            debug!("info: {:?}", info);
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "registration info successfully retrieved",
                    Some(json!({
                        "info": info
                    }))
                ));
        }
    }
}


async fn register_complete_get() -> impl Responder {
    info!("register_complete_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}


async fn register_complete_post(
    auth: web::Data<Auth>,
    params: web::Json<AuthRegistrationCompleteRequest>
) -> impl Responder {
    info!("register_complete_post()");

    match auth.complete_registration(&params.token, &params.password).await {
        Err(e) => {
            error!("an error occured while trying to complete the registration: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse::new(
                    false,
                    "an error occured while trying to complete the registration",
                    None
                ));
        }
        Ok(_) => {
            return HttpResponse::Ok()
                .json(ApiResponse::new(
                    true,
                    "successfully completed registration",
                    None
                ));
        }
    }
}

async fn auth_signin_get() -> impl Responder {
    info!("auth_signin_get()");

    return HttpResponse::Ok().body("Service is up. version: 1.0.0.0.dev");
}

async fn auth_signin_post(
    auth: web::Data<Auth>,
    params: web::Json<SignInRequest>
) -> impl Responder {
    info!("auth_signin_post()");

    match auth.user_authenticate(
        &params.email,
        &params.password
    ).await {
        Err(e) => {
            if matches!(e, AuthError::IncorrectUsernameAndPassword) {
                return HttpResponse::Ok()
                    .json(ApiResponse::new(
                        false,
                        "incorrect username and password combination",
                        None
                    ));
            } else {
                error!("unable to authenticate user: {:?}", e);
                return HttpResponse::InternalServerError()
                    .json(ApiResponse::new(
                        false,
                        "unable to authenticate user",
                        None
                    ));
            }
        }
        Ok(token) => {
            return HttpResponse::Ok()
                .append_header((AUTHORIZATION, format!("Bearer {}", token)))
                .json(ApiResponse::new(
                    true,
                    "user is authentic",
                    None
                ))
        }
    }
}
