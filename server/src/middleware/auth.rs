use log::{
    info,
    debug,
    error
};

use std::rc::Rc;


pub struct AuthUser {
    token: String
}

impl AuthUser {

    pub fn new(token: &str) -> Self {
        return Self {
            token: String::from(token)
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

        let service = self.service.clone();
        return Box::pin(async move {
            debug!("AuthMiddleware::call() [2]");

            let f = service.call(request);
            match f.await {
                Err(e) => {
                    error!("an error occured while handling the request: {:?}", e);
                    return Err(e);
                }
                Ok(result) => {
                    debug!("AuthMiddleware::call() result: {:?}", result);
                    return Ok(result);
                }
            }
        });
    }
}


