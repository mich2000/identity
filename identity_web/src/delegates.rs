use identity_service::store::Store;
use identity_service::service::person_service::get_user_info;
use identity_service::service::mail_service;
use identity_service::mail_struct::Report;
use identity_service::service::mail_service::MailTransport;
use identity_service::map_token_pwd::HashMapTokenPasswordChange;
use std::sync::Mutex;
use crate::IdentityError;

/**
 * Function that is used to send a welcome email to new users. If the smtp transport in the service library has not managed to make a real connection to the smtp server then it will return an error and a error will be logged.
 */
pub fn user_creation(id : &str, store : &Store, transport : &MailTransport) -> Result<(),IdentityError> {
    let user_info  = get_user_info(id, &store).ok_or(IdentityError::UserIsNotPresent)?;
    mail_service::send_email(transport,Report::new(user_info.get_email(), user_info.get_first_name(), 
    "Welcome to rust Identity",
    r#"
    Welcome new user

    Welcome to the rust identity server, this server is authentication backend that is written in Rust, uses JWT as authentication/authorization. It uses a embedded no-sql database named Sled. This database is very fast and efficiënt.

    Kind regards
    The admin
    "#)?)?;
    Ok(())
}

pub fn send_email_for_forgotten_pwd(token : &str,store : &Store, token_map : &Mutex<HashMapTokenPasswordChange>, transport : &MailTransport) -> Result<(), IdentityError> {
    let token_locked_map = token_map.lock().expect("Could not lock the token map which gaurds tokens for changing password");
    let user_id : String = token_locked_map.get_user_id_from_token(token).ok_or_else(|| IdentityError::CustomError("Could not get user id associated with the token.".to_owned()))?;
    let user = get_user_info(&user_id, &store).ok_or(IdentityError::UserIsNotPresent)?;
    mail_service::send_email(transport,Report::new(user.get_email(), user.get_first_name(), 
    "Welcome to rust Identity",
    &format!(r#"
    Dear user

    We recently received a notification that you forgot you're password. This token can be used to change your password. 

    Token: {}
    "#,&token))?)?;
    Ok(())
}