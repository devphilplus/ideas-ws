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

use configuration::{
    ProviderType,
    ApplicationConfiguration
};
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

        let data = data::Data::new(cfg.clone());

        // auth module
        let bind_host = cfg.bind_host.clone();
        let bind_port = cfg.bind_port.clone();

        let auth = auth::auth::Auth::new(
            cfg.clone(),
            mailer.clone(),
            tokenizer.clone(),
            data.clone()
        );

        let users = users::users::Users::new(
            cfg.clone(),
            mailer.clone(),
            tokenizer.clone(),
            data.clone()
        );

        let currencies = util::currencies::Currencies::new(
            cfg.clone(),
            data.clone()
        );

        let countries = util::countries::Countries::new(
            cfg.clone(),
            data.clone()
        );

        let people = people::people::People::new(
            cfg.clone(),
            data.clone()
        );

        let tenants = tenants::tenants::Tenants::new(
            cfg.clone(),
            data.clone()
        );

        let organizations = tenants::organizations::Organizations::new(
            cfg.clone(),
            data.clone()
        );

        let hr = hr::Hr::new(
            cfg.clone(),
            people.clone(),
            data.clone()
        );
        let employees = hr.employees();


        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(cfg.clone()))
                .app_data(web::Data::new(mailer.clone()))
                .app_data(web::Data::new(tokenizer.clone()))
                .app_data(web::Data::new(data.clone()))
                .app_data(web::Data::new(auth.clone()))
                .app_data(web::Data::new(users.clone()))
                .app_data(web::Data::new(currencies.clone()))
                .app_data(web::Data::new(countries.clone()))
                .app_data(web::Data::new(people.clone()))
                .app_data(web::Data::new(tenants.clone()))
                .app_data(web::Data::new(organizations.clone() ))

                // .app_data(web::Data::new(hr.clone()))
                .app_data(web::Data::new(employees.clone()))
                
                .wrap(crate::middleware::cors::CORS::new())
                .wrap(crate::middleware::auth::AuthUser::new(&cfg))

                .service(web::scope("/status").configure(crate::endpoints::status::config))
                .service(web::scope("/countries").configure(crate::endpoints::common::countries::config))
                .service(web::scope("/currencies").configure(crate::endpoints::common::currencies::config))
                .service(web::scope("/auth").configure(crate::endpoints::auth::config))
                .service(web::scope("/user").configure(crate::endpoints::user::config))
                // .service(web::scope("/clients").configure(crate::endpoints::clients::client::config))

                .service(web::scope("/tenants").configure(crate::endpoints::tenants::tenants::config))
                .service(web::scope("/organizations").configure(crate::endpoints::tenants::organizations::config))

                .service(web::scope("accounting/accounts").configure(crate::endpoints::accounting::accounts::config))

                // .service(web::scope("/crms/people").configure(crate::endpoints::crms::people::config))

                .service(web::scope("/hr/employees").configure(crate::endpoints::hr::employees::config))
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
