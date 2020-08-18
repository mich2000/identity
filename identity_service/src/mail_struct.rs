use crate::IdentityError;

use lettre_email::EmailBuilder;

pub struct Report {
    recipient : String,
    alias : String,
    subject : String,
    message : String
}

impl Report {
    pub fn new(recipient_email : &str, alias_name : &str, subject_string : &str, msg : &str) -> Result<Self, IdentityError> {
        if !identity_dal::util::control_email(recipient_email) {
            return Err(IdentityError::EmailNotCorrectFormat)
        }
        if subject_string.is_empty() {
            return Err(IdentityError::CustomError("Subject of the email is empty".to_string()))
        }
        Ok(
            Report {
                recipient : recipient_email.to_owned(),
                alias : alias_name.to_owned(),
                subject : subject_string.to_owned(),
                message : msg.to_owned()
            }
        )
    }

    // reference getter for the recipient property
    pub fn get_recipient(&self) -> &str { &self.recipient }

    // reference getter for the alias property
    pub fn get_alias(&self) -> &str { &self.alias }

    // reference getter for the subject property
    pub fn get_subject(&self) -> &str { &self.subject }

    // reference getter for the message property
    pub fn get_message(&self) -> &str { &self.message }

    pub fn email(&self) -> EmailBuilder {
        EmailBuilder::new()
        .to((self.get_recipient(), self.get_alias()))
        .subject(self.get_subject())
        .body(self.get_message())
    }
}