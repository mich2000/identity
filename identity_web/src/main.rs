#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate log;

use env_logger::Env;

mod controllers;
use controllers::auth_controller;
use controllers::error_controller;
use controllers::admin_controller;
use identity_service::store::StoreManager;
use identity_service::store::Store;
use identity_service::service::person_service::get_user_info;

pub type IdentityError = identity_service::IdentityError;

fn user_creation(id : &str, store : &Store) -> Result<(),IdentityError> {
    let user_info  = get_user_info(id, &store).ok_or(IdentityError::UserIsNotPresent)?;
    info!("name: {}", user_info.get_first_name());
    Ok(())
}

fn rocket() -> rocket::Rocket {
    let store = StoreManager::new(Some(user_creation));
    match &store.control_setup() {
        Ok(_) => rocket::ignite()
        .mount("/user", auth_controller::routes())
        .mount("/admin", admin_controller::routes())
        .register(error_controller::catches())
        .manage(store),
        Err(e) => panic!("error: {}",e)
    }
}

fn main() {
    env_logger::init_from_env(Env::default().filter_or("MY_LOG_LEVEL", "info").write_style_or("MY_LOG_STYLE", "always"));
    rocket().launch();
}