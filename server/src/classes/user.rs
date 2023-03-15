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
    }
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


#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    id: uuid::Uuid,
    email: String
}

impl CurrentUser {

    pub fn new(
        email: &str
    ) -> Self {
        return Self {
            id: uuid::Uuid::nil(),
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
        return self.id.is_nil() && self.email != "";
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

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        debug!("CurrentUser::from_request");
        
        let user = CurrentUser::anonymous();
        if let Some(header_value) = request.headers().get(header::AUTHORIZATION) {
            let token_value = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();

            if token_value != "" {
                if let Some(tokenizer) = request.app_data::<web::Data<Tokenizer>>() {
                    if tokenizer.is_valid(&token_value) {
                        if let Ok(claims) = tokenizer.get_claims(&token_value) {
                            debug!("claims: {:?}", claims);
                            let email = claims.email().clone();
                            let email_str = email.as_str();
                            return ok(CurrentUser::new(&email_str));
                        } else {
                            debug!("unable to retrieve claims");
                        }
                    } else {
                        debug!("token is invalid: {:?}", token_value);
                    }
                }
            }
        }

        return ok(user);
    }
}