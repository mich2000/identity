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

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .register(error_controller::catches())
    .mount("/user", auth_controller::routes())
    .mount("/admin", admin_controller::routes())
    .manage(StoreManager::new_with_setup())
    .manage(mail_service::get_transport())
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or("MY_LOG_LEVEL", "info").write_style_or("MY_LOG_STYLE", "always"));
    rocket().launch();
}