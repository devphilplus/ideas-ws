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


#[derive(Clone)]
pub struct Mailer {
    relay_host: String,
    credentials: Credentials,
    transport: Option<SmtpTransport>
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
            credentials: creds,
            transport: None
        }
    }

    pub fn connect(&mut self) -> Result<(), MailerError> {
        match SmtpTransport::relay(&self.relay_host) {
            Err(e) => {
                error!("unable to create relay: {:?}", e);
                return Err(MailerError::ConfigurationError(String::from("unable to create relay")));
            }
            Ok(result) => {
                let transport = result.credentials(self.credentials.clone())
                    .build();
                self.transport = Some(transport);

                return Ok(());
            }
        }           
    }

    pub fn send(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str
    ) -> Result<(), MailerError> {
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
                    if let Some(transport) = &self.transport {
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
                    } else {
                        error!("call connect first before sending");
                        return Err(MailerError::SendError(String::from("connect first before sending")));
                    }
                }
            }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send() {

        let mut mailer = Mailer::new(
            "smtp.gmail.com",
            "beowulf1416@gmail.com",
            // if using smtp gmail, the password below should be generated
            // using App Passwords in Google Account Settings
            "vfmvieprohfwwvvf"
        );

        if let Err(e) = mailer.connect() {
            error!("error: {:?}", e);
            assert!(false);
        } else {
            if let Err(e) = mailer.send(
                "beowulf1416@gmail.com",
                "ferdinand@marginfuel.com",
                "testing",
                "this is a test"
            ) {
                error!("error: {:?}", e);
                assert!(false);
            } else {
                assert!(true);
            }
        }
    }
}
