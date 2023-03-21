use std::pin::Pin;

use actix_http::header::HeaderMap;
use configuration::ApplicationConfiguration;
use log::{
    info,
    debug,
    error
};

use futures::{
    future::{
        ok,
        err,
        Ready
    }, Future
};

use http::{
    method::Method,
    header
};

use actix_web::{
    dev::Payload,
    http::StatusCode, 
    HttpMessage,
    HttpRequest,
    FromRequest,
    ResponseError,
    web
};
use serde::{
    Serialize,
    Deserialize
};
use tokenizer::Tokenizer;

use common::user::User;
use users::users::Users;


#[derive(Debug)]
pub enum UserError {
    InternalServerError
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserError::InternalServerError => write!(f, "internal server error")
        }
    }
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentUser {
    id: uuid::Uuid,
    email: String
}

impl CurrentUser {

    pub fn new(
        id: &uuid::Uuid,
        email: &str
    ) -> Self {
        return Self {
            id: id.clone(),
            email: String::from(email)
        };
    }

    pub fn anonymous() -> Self {
        return Self {
            id: uuid::Uuid::nil(),
            email: String::from("")
        };
    }

    pub fn is_authenticated(&self) -> bool {
        return !self.id.is_nil() && self.email != "";
    }

    pub fn id(&self) -> uuid::Uuid {
        return self.id.clone();
    }

    pub fn email(&self) -> String {
        return self.email.to_owned();
    }
}


impl FromRequest for CurrentUser {
    type Error = UserError;
    type Future = Ready<Result<Self, Self::Error>>;
    // type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        debug!("CurrentUser::from_request");
        
        // let sr = request.clone();
        // return Box::pin(async move {
        //     let mut user = CurrentUser::anonymous();
        //     let mut token_value = String::from("");
        //     let mut email = String::from("");

        //     if let Some(header_value) = sr.headers().get(header::AUTHORIZATION) {
        //         token_value = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();
        //         debug!("token_value: {:?}", token_value);
        //     }

        //     if !token_value.is_empty() {
        //         if let Some(tokenizer) = sr.app_data::<web::Data<Tokenizer>>() {
        //             if tokenizer.is_valid(&token_value) {
        //                 if let Ok(claims) = tokenizer.get_claims(&token_value) {
        //                     email = claims.email().clone();
        //                 }
        //             }
        //         }
        //     }

        //     if !email.is_empty() {
        //         if let Some(users) = sr.app_data::<web::Data<Users>>() {
        //             match users.user_by_email(&email.as_str()).await {
        //                 Err(e) => {
        //                     error!("unable to retrieve user data: {:?}", e);
        //                 }
        //                 Ok(user_data) => {
        //                     user = CurrentUser::new(
        //                         &user_data.id(),
        //                         &user_data.email()
        //                     );
        //                 }
        //             }
        //         }
        //     }

        //     return Ok(user);
        // });

        if let Some(user) = request.extensions().get::<CurrentUser>() {
        // if let Some(user) = request.app_data::<CurrentUser>() {
            return ok(user.clone());
        } else {
            error!("current user not found");
            return ok(CurrentUser::anonymous());
        }
    }
}