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
pub struct User {
    email: String
}

impl User {

    pub fn new(
        email: &str
    ) -> Self {
        return Self {
            email: String::from(email)
        };
    }

    pub fn anonymous() -> Self {
        return Self {
            email: String::from("")
        };
    }

    pub fn is_authenticated(&self) -> bool {
        return self.email != "";
    }
}


impl FromRequest for User {
    type Error = UserError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        debug!("User::from_request");
        
        let user = User::anonymous();
        if let Some(header_value) = request.headers().get(header::AUTHORIZATION) {
            let token_value = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();

            if token_value == "" {
                if let Some(tokenizer) = request.app_data::<web::Data<Tokenizer>>() {
                    if tokenizer.validate(&token_value) {
                        if let Ok(claims) = tokenizer.get_claims(&token_value) {
                            debug!("claims: {:?}", claims);
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