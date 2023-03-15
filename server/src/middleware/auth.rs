use log::{
    info,
    debug,
    error
};

use std::rc::Rc;
use std::task::{ Context, Poll };
use std::future::{ ready, Ready };
use futures::future::LocalBoxFuture;

use http::{
    method::Method,
    header
};

use actix_web::{
    HttpMessage,
    error::Error,
    dev::{
        Service, 
        Transform, 
        ServiceRequest, 
        ServiceResponse
    },
    web
};

use configuration::ApplicationConfiguration;
use crate::classes::user::CurrentUser;


pub struct AuthUser {
    configuration: ApplicationConfiguration
}

impl AuthUser {

    pub fn new(
        configuration: &ApplicationConfiguration
    ) -> Self {
        return Self {
            configuration: configuration.clone()
        };
    }
}


pub struct AuthMiddleWare<S> {
    service: Rc<S>
}


impl <S, B> Transform<S, ServiceRequest> for AuthUser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleWare<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(AuthMiddleWare {
            service: Rc::new(service)
        }));
    }
}


impl <S, B> Service<ServiceRequest> for AuthMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(context);
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        debug!("AuthMiddleware::call() [1]");

        // debug!("request: {:?}", request);

        let path = request.match_info();
        debug!("path: {:?}", path);

        let service = self.service.clone();
        return Box::pin(async move {
            debug!("AuthMiddleware::call() [2]");

            // let mut user = User::new();

            // if request.method() == Method::POST {
            //     if let Some(header_value) = request.headers().get(header::AUTHORIZATION) {
            //         let token_value = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();


            //     }
            // }

            // request.extensions_mut().insert(user);

            let f = service.call(request);
            match f.await {
                Err(e) => {
                    error!("an error occured while handling the request: {:?}", e);
                    return Err(e);
                }
                Ok(service_result) => {
                    // debug!("AuthMiddleware::call() result: {:?}", service_result);
                    return Ok(service_result);
                }
            }
        });
    }
}


