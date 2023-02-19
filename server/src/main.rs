extern crate log;

mod classes;
mod endpoints;

use std::sync::Arc;
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



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting up...");

    if let Some(cfg) = ApplicationConfiguration::get() {
        debug!("configuration: {:?}", cfg);

        // auth module
        let bind_host = cfg.bind_host.clone();
        let bind_port = cfg.bind_port.clone();

        let result_auth = auth::auth::Auth::new(cfg.clone());
        if let Err(e) = result_auth {
            error!("unable to create auth object");
            return Err(Error::new(ErrorKind::Other, "unable to create auth object"));
        }
        let auth = result_auth.unwrap();

        // mailer
        let mailer = mailer::Mailer::new(
            &cfg.mail.host,
            &cfg.mail.user,
            &cfg.mail.password
        );

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(cfg.clone()))
                .app_data(web::Data::new(mailer.clone()))
                .app_data(web::Data::new(auth.clone()))
                
                .service(web::scope("/status").configure(crate::endpoints::status::config))
                .service(web::scope("/common").configure(crate::endpoints::status::config))
                .service(web::scope("/auth").configure(crate::endpoints::auth::config))
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
