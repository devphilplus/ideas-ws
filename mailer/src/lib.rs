use log::{
    info,
    error
};

use lettre::{
    Message, 
    SmtpTransport, 
    Transport,
    transport::{smtp::authentication::Credentials},
    message::{
        header,
        MultiPart,
        SinglePart
    }
};


#[derive(Debug)]
pub enum MailerError {
    ConfigurationError(String),
    SendError(String)
}


#[derive(Debug, Clone)]
pub struct Mailer {
    relay_host: String,
    credentials: Credentials
}


impl Mailer {

    pub fn new(
        smtp_host: &str,
        smtp_user: &str,
        smtp_pw: &str
    ) -> Self {
        let creds = Credentials::new(
            String::from(smtp_user),
            String::from(smtp_pw)
        );
        return Self {
            relay_host: String::from(smtp_host),
            credentials: creds
        }
    }

    // pub fn connect(&mut self) -> Result<(), MailerError> {
    //     match SmtpTransport::relay(&self.relay_host) {
    //         Err(e) => {
    //             error!("unable to create relay: {:?}", e);
    //             return Err(MailerError::ConfigurationError(String::from("unable to create relay")));
    //         }
    //         Ok(result) => {
    //             let transport = result.credentials(self.credentials.clone())
    //                 .build();
    //             self.transport = Some(transport);

    //             return Ok(());
    //         }
    //     }           
    // }

    pub fn send(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str
    ) -> Result<(), MailerError> {
        match SmtpTransport::relay(&self.relay_host) {
            Err(e) => {
                error!("unable to create relay: {:?}", e);
                return Err(MailerError::ConfigurationError(String::from("unable to create relay")));
            }
            Ok(tb) => {
                let transport = tb.credentials(self.credentials.clone()).build();

                match Message::builder()
                    .from(from.parse().unwrap())
                    .to(to.parse().unwrap())
                    .reply_to(to.parse().unwrap())
                    .subject(subject)
                    .multipart(
                        MultiPart::alternative()
                            .singlepart(
                                SinglePart::builder()
                                    .header(header::ContentType::TEXT_PLAIN)
                                    .body(String::from(body))
                            )
                            .singlepart(
                                SinglePart::builder()
                                    .header(header::ContentType::TEXT_HTML)
                                    .body(String::from(body))
                            )
                    ) {
                        Err(e) => {
                            error!("unable to send email: {:?}", e);
                            return Err(MailerError::SendError(e.to_string()));
                        }
                        Ok(email) => {
                            match transport.send(&email) {
                                Err(e) => {
                                    error!("unable to send email: {:?}", e);
                                    return Err(MailerError::SendError(e.to_string()));
                                }
                                Ok(_) => {
                                    info!("email sent");
                                    return Ok(());
                                }
                            }
                        }
                    }
            }
        };
    }
}


#[cfg(test)]
mod tests {
    use configuration::ApplicationConfiguration;

    use super::*;

    

    #[test]
    fn send() {
        env_logger::init();

        if let Some(cfg) = ApplicationConfiguration::get() {
            let mailer = Mailer::new(
                &cfg.mailer.host,
                &cfg.mailer.user,
                // if using smtp gmail, the password below should be generated
                // using App Passwords in Google Account Settings
                &cfg.mailer.password
            );

            if let Err(e) = mailer.send(
                "beowulf1416@gmail.com",
                "ferdinand@marginfuel.com",
                "testing",
                "this is a test"
            ) {
                error!("error: {:?}", e);
                assert!(false, "unable to send");
            } else {
                assert!(true);
            }
        } else {
            assert!(false, "unable to load configuration");
        }
    }
}
