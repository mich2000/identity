#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate log;

use env_logger::Env;

mod controllers;
use controllers::auth_controller;
use controllers::error_controller;
use controllers::admin_controller;

mod delegates;

use identity_service::store::StoreManager;
use identity_service::service::mail_service;

pub type IdentityError = identity_service::IdentityError;

use std::io::Cursor;
use rocket::http::{ ContentType, Status };
use rocket::Response;
use rocket::fairing::AdHoc;

#[get("/")]
fn get_handler<'a>() -> Response<'a> {
    let mut res = Response::new();
    res.set_status(Status::new(200, "No Content"));
    res.adjoin_header(ContentType::Plain);
    res.adjoin_raw_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
    res.adjoin_raw_header("Access-Control-Allow-Origin", "*");
    res.adjoin_raw_header("Access-Control-Allow-Credentials", "true");
    res.adjoin_raw_header("Access-Control-Allow-Headers", "Content-Type");
    res.set_sized_body(Cursor::new("Response")); 
    res
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .register(error_controller::catches())
    .mount("/user", auth_controller::routes())
    .mount("/admin", admin_controller::routes())
    .manage(StoreManager::new_with_setup())
    .manage(mail_service::get_transport())
    .attach(AdHoc::on_response("Cors", |_,res| {
        res.set_status(Status::new(200, "No Content"));
        res.adjoin_raw_header("Access-Control-Allow-Origin", "*");
        res.adjoin_raw_header("Access-Control-Allow-Methods", "POST, PUT, DELETE, GET, OPTIONS");
        res.adjoin_raw_header("Access-Control-Allow-Credentials", "true");
        res.adjoin_raw_header("Access-Control-Allow-Headers", "Content-Type");
    }))
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or("MY_LOG_LEVEL", "info").write_style_or("MY_LOG_STYLE", "always"));
    rocket().launch();
}