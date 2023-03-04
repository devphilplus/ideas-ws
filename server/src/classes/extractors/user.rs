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

use actix_web::{
    dev::Payload,
    http::StatusCode, 
    HttpMessage,
    HttpRequest,
    FromRequest, ResponseError,
};


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


#[derive(Debug)]
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


impl ResponseError for UserError {
    
    fn status_code(&self) -> StatusCode {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}


impl FromRequest for User {
    type Error = UserError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // if let Some(header_value) = request.headers().get(http::header::AUTHORIZATION) {

        // }

        return ok(User::anonymous());
    }
}