use rocket_contrib::json::JsonValue;
use rocket::Request;
use rocket::Catcher;
use crate::IdentityError;

pub fn catches() -> Vec<Catcher> {
    catchers![not_found]
}

/**
 * Returns a json value explicitely for errors.
 */
pub fn return_error_json(error_message : IdentityError) -> JsonValue {
    warn!("{}",error_message);
    json!({
        "Status" : "NOT OK",
        "Message" : format!("{}",error_message)
    })
}

/**
 * Catches the 404 error code, this means that this url doesn't exist.
 */
#[catch(404)]
pub fn not_found(req: &Request) -> JsonValue {
    warn!("Path: {} is not valid", req.uri());
    return_error_json(IdentityError::CustomError(format!("Sorry, '{}' is not a valid path.", req.uri())))
}