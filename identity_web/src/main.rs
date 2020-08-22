#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate log;

use env_logger::Env;

mod controllers;
use controllers::auth_controller;
use controllers::error_controller;
use controllers::admin_controller;
use controllers::basic_controller;

mod counter;
mod adhoc;
mod delegates;

use counter::Counter;
use identity_service::store::StoreManager;
use identity_service::service::mail_service;
use std::sync::{Arc,Mutex};

pub type IdentityError = identity_service::IdentityError;
pub type SharedCounter = Arc<Mutex<Counter>>;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .register(error_controller::catches())
    .mount("/", basic_controller::routes())
    .mount("/user", auth_controller::routes())
    .mount("/admin", admin_controller::routes())
    .manage(StoreManager::new_with_setup())
    .manage(mail_service::get_transport())
    .manage(Arc::new(Mutex::new(Counter::default())))
    .attach(adhoc::cors_handler())
    .attach(adhoc::count_handler())
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or("MY_LOG_LEVEL", "info")
    .write_style_or("MY_LOG_STYLE", "always"));
    rocket().launch();
}