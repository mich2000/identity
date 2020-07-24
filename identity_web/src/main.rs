#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate log;

use env_logger::Env;

mod controllers;
use controllers::auth_controller;
use controllers::error_controller;
use controllers::admin_controller;

mod fairings;
use fairings::counter;
use identity_service::store::StoreManager;

fn rocket() -> rocket::Rocket {
    let store = StoreManager::new();
    match &store.control_setup() {
        Ok(_) =>rocket::ignite()
        .mount("/user", auth_controller::routes())
        .mount("/admin", admin_controller::routes())
        .register(error_controller::catches())
        .manage(store)
        .attach(counter::HitCount::default()),
        Err(e) => panic!("error: {}",e)
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or("MY_LOG_LEVEL", "info").write_style_or("MY_LOG_STYLE", "always"));
    rocket().launch();
}