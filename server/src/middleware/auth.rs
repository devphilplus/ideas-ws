use common::tenant::Tenant;
use log::{
    info,
    debug,
    error
};
use tokenizer::Tokenizer;
use users::users::Users;

use std::default;
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

            let mut user = CurrentUser::anonymous();
            if request.method() == Method::POST {
                let mut token_value = String::from("");
                let mut email = String::from("");
                let mut tenant_id: uuid::Uuid = uuid::Uuid::nil();

                if let Some(header_value) = request.headers().get(header::AUTHORIZATION) {
                    token_value = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();
                }

                if !token_value.is_empty() {
                    if let Some(tokenizer) = request.app_data::<web::Data<Tokenizer>>() {
                        if tokenizer.is_valid(&token_value) {
                            if let Ok(claims) = tokenizer.get_claims(&token_value) {
                                email = claims.email().clone();
                                tenant_id = claims.tenant().clone();
                            }
                        }
                    } else {
                        error!("tokenizer not found");
                    }
                }

                if !email.is_empty() {
                    if let Some(users) = request.app_data::<web::Data<Users>>() {
                        match users.user_by_email(&email.as_str()).await {
                            Err(e) => {
                                error!("unable to retrieve user data: {:?}", e);
                            }
                            Ok(user_data) => {
                                let user_id = user_data.id();

                                let mut tenants: Vec<Tenant> = Vec::new();
                                if let Ok(mut result) = users.user_tenants(&user_id).await {
                                    // debug!("result: {:?}", result);
                                    tenants.append(&mut result);
                                }

                                let mut default_tenant_id: uuid::Uuid = uuid::Uuid::nil();
                                if tenant_id.is_nil() {
                                    if let Ok(result) = users.user_tenant_default(&user_id).await {
                                        debug!("result: {:?}", result);
                                        default_tenant_id = result.id();
                                    }
                                } else {
                                    default_tenant_id = tenant_id;
                                }

                                user = CurrentUser::new(
                                    &user_data.id(),
                                    &user_data.email(),
                                    &default_tenant_id,
                                    tenants
                                );
                            }
                        }
                    } else {
                        error!("users not found");
                    }
                }
            }
            request.extensions_mut().insert(user);

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


