pub mod auth;
mod data;

#[cfg(test)]
mod tests {
    use super::*;

    use rand::prelude::*;
    use uuid::Uuid;

    use configuration::ApplicationConfiguration;

    #[test]
    fn auth_register() {
        if let Some(cfg) = ApplicationConfiguration::get() {
            let mut rng = rand::thread_rng();
            let i: i32 = rng.gen::<i32>();

            let token = Uuid::new_v4();
            let email = format!("test_{}@test.com", i);

            if let Ok(auth) = auth::Auth::new(cfg) {
                if let Err(e) = auth.register(&token, &email) {
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
