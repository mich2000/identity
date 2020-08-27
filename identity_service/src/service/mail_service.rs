use lettre::{ SmtpTransport, Transport };
use lettre::smtp::authentication::Credentials;
use crate::mail_struct::Report;
use crate::IdentityError;
use std::sync::Mutex;
use crate::util::get_value_from_key;

lazy_static! {
    /**
     * Variable that is used to get the email that will be used as the smtp client.
     */
    static ref EMAIL : String = get_value_from_key("PERSON_SMTP_USERNAME")
    .expect("PERSON_SMTP_USERNAME variable not found in the .env config file or as environment variable");
}

pub type MailTransport = Mutex<SmtpTransport>;

/**
 * Returns the smtp transport that can later be used then
 */
pub fn get_transport() -> MailTransport {
    Mutex::new(lettre::SmtpClient::new_simple(
        &get_value_from_key("PERSON_SMTP_DOMAIN")
        .expect("PERSON_SMTP_DOMAIN variable not found in the .env config file or as environment variable")
    )
    .expect("stmp domain is not good")
    .credentials(
        Credentials::new(EMAIL.to_string(),
        get_value_from_key("PERSON_SMTP_PASSWORD")
        .expect("PERSON_SMTP_PASSWORD variable not found in the .env config file or as environment variable"))
    )
    .transport())
}

/**
 * Function that takes in a report Structure that it then uses to send a email.
 */
pub fn send_email(transport : &MailTransport, report : Report) -> Result<(),IdentityError> {
    let mail = match report.email().from(EMAIL.clone()).build() {
        Ok(mail) => mail,
        Err(_) => return Err(IdentityError::CustomError("Faulthy email structure.".to_string()))
    };
    match transport.lock().expect("Could not lock the email transport").send(mail.into()) {
        Ok(_) => {
            info!("Email has been sent through the SMTP transport bus.");
            Ok(())
        },
        Err(_) => {
            warn!("{}",IdentityError::CouldNotSendEmail);
            Err(IdentityError::CouldNotSendEmail)
        }
    }
}