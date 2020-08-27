use rocket_contrib::json::JsonValue;
use rocket::Request;
use rocket::Catcher;
use crate::IdentityError;

pub fn catches() -> Vec<Catcher> {
    catchers![ 
        not_found,
        internal_error,
        unprocessable_entity
    ]
}

/**
 * Returns a json value of error that are being send. When the parameter grave_error is set to true it logged it as an error and if it is false it is logged as warn.
 */
pub fn return_error_json(error_message : IdentityError, grave_error : bool) -> JsonValue {
    if grave_error {
        error!("{}",error_message)
    } else {
        warn!("{}",error_message);
    }
    json!({
        "ok" : false,
        "error" : format!("{}",error_message)
    })
}

/**
 * Catches the 404 error code, this means that the path doesn't exist.
 */
#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    return_error_json(IdentityError::CustomError(format!("Sorry, '{}' is not a valid path.", req.uri())),false)
}

/**
 * Catches the 422 error code, this means that the path doesn't exist.
 */
#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    return_error_json(IdentityError::CustomError("Given entity could not be processed.".to_string()),false)
}

#[catch(500)]
fn internal_error(req : &Request) -> JsonValue {
    return_error_json(IdentityError::CustomError(format!("An internal error has happened. Path : {}", req.uri())),true)
}