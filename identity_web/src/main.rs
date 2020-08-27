#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate log;

mod controllers;
use controllers::auth_controller;
use controllers::error_controller;
use controllers::admin_controller;
use controllers::basic_controller;

mod counter;
mod adhoc;
mod delegates;
mod key;

use counter::Counter;
use std::sync::Mutex;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub type IdentityError = identity_service::IdentityError;
pub type SharedCounter = Mutex<Counter>;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .register(error_controller::catches())
    .mount("/", basic_controller::routes())
    .mount("/user", auth_controller::routes())
    .mount("/admin", admin_controller::routes())
    .manage(identity_service::store::StoreManager::new_with_setup())
    .manage(identity_service::service::mail_service::get_transport())
    .manage(identity_service::map_token_pwd::get_mutext_token_forgotten_pwd_map())
    .manage(Mutex::new(Counter::default()))
    .attach(adhoc::cors_handler())
    .attach(adhoc::count_handler())
}

fn log() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_config(
        Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(
            ConsoleAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{l}: {d(%Y-%m-%d %H:%M:%S)} => {m}{n}")))
                .build()
        )))
        .appender(Appender::builder().build("requests", Box::new(
            FileAppender::builder()
                .append(true)
                .encoder(Box::new(PatternEncoder::new("{l}: {d(%Y-%m-%d %H:%M:%S)} => {m}{n}")))
                .build(identity_service::util::get_value_from_key("LOG_FILE").unwrap_or_else(|| "log/requests.log".to_string()))?
        )))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("requests")
                .build(LevelFilter::Info)
        )?
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log()?;
    rocket().launch();
    Ok(())
}