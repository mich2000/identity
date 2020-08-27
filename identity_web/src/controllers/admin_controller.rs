use rocket_contrib::json::{Json,JsonValue};
use super::error_controller;
use identity_service::store::StoreManager;
use identity_service::viewmodels::admin::create_user::AdminCreateUserViewModel;
use identity_service::viewmodels::admin::delete_user::DeleteUserViewModel;
use identity_service::viewmodels::admin::update_user_pwd::AdminChangePasswordUserViewModel;
use identity_service::viewmodels::admin::update_user::AdminUpdateUserViewModel;
use identity_service::service::admin_service;
use crate::key::ApiKey;
use rocket::State;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![ 
        register_user, 
        delete_user, 
        change_password, 
        update_user,
        all_users
    ]
}

/**
 * Admin function used to register a new user with the help of the viewmodel AdminCreateUserViewModel, sends a json back to notify the requester if his request was succesfull or not.
*/
#[post("/registration", format = "application/json", data = "<model>")]
fn register_user(key : ApiKey, model : Json<AdminCreateUserViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match admin_service::create_user(key.get_key(),model.0, &sled_db.give_unique_id(),sled_db.give_store()) {
        Ok(_) => {
            info!("Admin has added user has been added");
            json!({
                "ok" : true,
                "message" : "User has been added"
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Admin function used to update an user's email, first and last anem with the help of the viewmodel AdminUpdateUserViewModel, sends a json back to notify the requester if his request was succesfull or not.
*/
#[put("/update", format = "application/json", data = "<model>")]
fn update_user(key : ApiKey, model : Json<AdminUpdateUserViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match admin_service::update_user(key.get_key(),model.0, sled_db.give_store()) {
        Ok(_) => {
            info!("Admin has successfully been updated an user");
            json!({
                "ok" : true,
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Admin function used to delete an user, this will use user id in the viewmodel DeleteUserViewModel. Controls if the id exists or not and delete if it does. An error is thrown whent the token is empty or the user couldn't be deleted.
*/
#[post("/delete", format = "application/json", data = "<model>")]
fn delete_user(key : ApiKey,model : Json<DeleteUserViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match admin_service::delete_user(key.get_key(),model.0,sled_db.give_store()) {
        Ok(_) => {
            info!("Admin has been deleted user has been added");
            json!({
                "ok" : true,
                "message" : "User has been deleted"
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Admin function changing the password of an user with the help of the viewmodel AdminChangePasswordUserViewModel,sends a json back to notify the requester if his request was succesfull or not.
*/
#[put("/password", format = "application/json", data = "<model>")]
fn change_password(key : ApiKey, model : Json<AdminChangePasswordUserViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match admin_service::update_user_pwd(key.get_key(),model.0,sled_db.give_store()) {
        Ok(_) => {
            info!("Admin has changed the password of an user has been changed.");
            json!({
                "ok" : true,
                "message" : "User password has sucessfully been changed"
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * returns a json object where basic information of all non admin users is presented in an array.
 */
#[post("/users", format = "application/json")]
fn all_users(key : ApiKey,sled_db : State<StoreManager>) -> JsonValue {
    match admin_service::get_all_users(key.get_key(),sled_db.give_store()) {
        Ok(users) => {
            info!("Admin has asked a json object of all users within.");
            json!(users)
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}