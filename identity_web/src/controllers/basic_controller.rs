use rocket::{Route,State};
use rocket::response::NamedFile;
use rocket_contrib::json::JsonValue;
use crate::SharedCounter;
use crate::controllers::error_controller;
use crate::IdentityError;

pub fn routes() -> Vec<Route> {
    routes![ 
        get_count,
        favicon
    ]
}

#[get("/counter")]
fn get_count(counter : State<SharedCounter>) -> JsonValue {
    match counter.lock() {
        Ok(count) => {
            json!({
                "GET" : count.get(),
                "POST" : count.post(),
                "PUT" : count.put(),
                "DELETE" : count.delete(),
            })
        },
        Err(_) => error_controller::return_error_json(IdentityError::CustomError("Could not get the counter".to_owned()), false)
    }
}

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open("static/favicon.ico").ok()
}