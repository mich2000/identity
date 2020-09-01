use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use crate::IdentityError;

static HEADER_API_KEY : &str = "X-API-Key";

pub struct ApiKey(String);

impl ApiKey {
    pub fn get_key(&self) -> &str {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = IdentityError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one(HEADER_API_KEY) {
            Some(key) => Outcome::Success(ApiKey(key.to_owned())),
            None => Outcome::Failure((Status::new(400, "Token has not been given in the headers"),IdentityError::CustomError("Token has not been given in the headers".to_owned())))
        }
    }
}