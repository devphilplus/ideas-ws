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

        // user module
        let result_users = users::users::Users::new(
            cfg.clone(),
            mailer.clone(),
            tokenizer.clone()
        );
        if let Err(e) = result_users {
            error!("unable to create users object: {:?}", e);
            return Err(Error::new(ErrorKind::Other, "unable to create users object"));
        }
        let users = result_users.unwrap();

        // clients module
        // let result_clients = clients::clients::Clients::new(
        //     cfg.clone()
        // );
        // if let Err(e) = result_clients {
        //     error!("unable to create clients object: {:?}", e);
        //     return Err(Error::new(ErrorKind::Other, "unable to create clients object"));
        // }
        // let clients = result_clients.unwrap();

        // tenants module
        let result_tenants = tenants::tenants::Tenants::new(cfg.clone());
        if let Err(e) = result_tenants {
            error!("unable to create tenants object: {:?}", e);
            return Err(Error::new(ErrorKind::Other, "unable to create tenants object"));
        }
        let tenants = result_tenants.unwrap();


        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(cfg.clone()))
                .app_data(web::Data::new(mailer.clone()))
                .app_data(web::Data::new(tokenizer.clone()))
                .app_data(web::Data::new(auth.clone()))
                .app_data(web::Data::new(users.clone()))
                // .app_data(web::Data::new(clients.clone()))
                .app_data(web::Data::new(tenants.clone()))
                
                .wrap(crate::middleware::cors::CORS::new())
                .wrap(crate::middleware::auth::AuthUser::new(&cfg))

                .service(web::scope("/status").configure(crate::endpoints::status::config))
                // .service(web::scope("/countries").configure(crate::endpoints::common::countries::config))
                .service(web::scope("/auth").configure(crate::endpoints::auth::config))
                .service(web::scope("/user").configure(crate::endpoints::user::config))
                // .service(web::scope("/clients").configure(crate::endpoints::clients::client::config))
                .service(web::scope("/tenants").configure(crate::endpoints::tenants::tenants::config))
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
