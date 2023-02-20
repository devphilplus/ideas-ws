pub mod auth;
mod data;

#[cfg(test)]
mod tests {
    use super::*;

    use rand::prelude::*;
    use uuid::Uuid;

    use configuration::ApplicationConfiguration;

    #[actix_rt::test] 
    async fn auth_register() {
        env_logger::init();
        
        if let Some(cfg) = ApplicationConfiguration::get() {
            // mailer
            let mailer = mailer::Mailer::new(
                &cfg.mailer.host,
                &cfg.mailer.user,
                &cfg.mailer.password
            );

            let mut rng = rand::thread_rng();
            let i: i32 = rng.gen::<i32>();

            let token = Uuid::new_v4();
            let email = format!("test_{}@mailinator.com", i);

            if let Ok(auth) = auth::Auth::new(cfg, mailer) {
                if let Err(e) = auth.register(&token, &email).await {
                    assert!(false, "error occured while registering");
                } else {
                    assert!(true);
                }
            } else {
                assert!(false, "error occured while creating an instance of Auth");
            }
        } else {
            assert!(false, "unable to retrieve configuration");
        }
    }
}
