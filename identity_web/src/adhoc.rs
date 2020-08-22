use rocket::{Outcome, State, http::{Status, Method}};
use rocket::fairing::AdHoc;
use crate::SharedCounter;

pub fn count_handler() -> AdHoc {
    AdHoc::on_request("Counter middleware", |req : &mut rocket::Request,_| {
        match req.guard::<State<SharedCounter>>() {
            Outcome::Success(counter) => {
                match req.method() {
                    Method::Get => counter.lock().unwrap().increment_get(),
                    Method::Post => counter.lock().unwrap().increment_post(),
                    Method::Put => counter.lock().unwrap().increment_put(),
                    Method::Delete => counter.lock().unwrap().increment_delete(),
                    _ => return
                };
            },
            Outcome::Failure(_) => return,
            Outcome::Forward(_) => return
        };
    })
}

pub fn cors_handler() -> AdHoc {
    AdHoc::on_response("Cors handler", |_,res| {
        res.set_status(Status::new(200, "No Content"));
        res.adjoin_raw_header("Access-Control-Allow-Origin", "*");
        res.adjoin_raw_header("Access-Control-Allow-Methods", "POST, PUT, DELETE, GET");
        res.adjoin_raw_header("Access-Control-Allow-Credentials", "true");
        res.adjoin_raw_header("Access-Control-Allow-Headers", "Content-Type");
    })
}