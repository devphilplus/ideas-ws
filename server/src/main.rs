extern crate log;

mod classes;
mod middleware;
mod endpoints;

// use std::sync::Arc;
use std::io::{
    Error,
    ErrorKind
};

use log::{
    info,
    error,
    debug
};

use actix_web::{
    HttpServer,
    App,
    web
};

use configuration::ApplicationConfiguration;
use tokenizer::Tokenizer;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting up...");

    if let Some(cfg) = ApplicationConfiguration::get() {
        debug!("configuration: {:?}", cfg);
        // tokenizer
        let tokenizer = Tokenizer::new(&cfg.jwt.secret);

        // mailer
        let mailer = mailer::Mailer::new(
            &cfg.mailer.host,
            &cfg.mailer.user,
            &cfg.mailer.password
        );

        // auth module
        let bind_host = cfg.bind_host.clone();
        let bind_port = cfg.bind_port.clone();

        let result_auth = auth::auth::Auth::new(
            cfg.clone(), 
            mailer.clone(),
            tokenizer.clone()
        );

        if let Err(e) = result_auth {
            error!("unable to create auth object: {:?}", e);
            return Err(Error::new(ErrorKind::Other, "unable to create auth object"));
        }
        let auth = result_auth.unwrap();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(cfg.clone()))
                .app_data(web::Data::new(mailer.clone()))
                .app_data(web::Data::new(tokenizer.clone()))
                .app_data(web::Data::new(auth.clone()))
                
                .wrap(crate::middleware::cors::CORS::new())
                .wrap(crate::middleware::auth::AuthUser::new(&cfg))

                .service(web::scope("/status").configure(crate::endpoints::status::config))
                .service(web::scope("/common").configure(crate::endpoints::status::config))
                .service(web::scope("/auth").configure(crate::endpoints::auth::config))
                .service(web::scope("/user").configure(crate::endpoints::user::config))
        })
        .workers(2)
        .bind(format!("{}:{}", bind_host, bind_port))?
        .run();
    
        info!("listening on http://{}:{}", bind_host, bind_port);
        return server.await;
    } else {
        error!("no configuration loaded");
    }

    info!("exiting");
    return Ok(());
}
