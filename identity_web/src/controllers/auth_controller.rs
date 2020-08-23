use rocket_contrib::json::{Json,JsonValue};
use super::error_controller;
use identity_service::service::person_service;
use identity_service::store::StoreManager;
use identity_service::viewmodels::auth::registration::RegistrationViewModel;
use identity_service::viewmodels::auth::change_pwd::ChangeForgottenPassword;
use identity_service::viewmodels::auth::login::LoginViewModel;
use identity_service::viewmodels::auth::token::TokenHolderViewModel;
use identity_service::viewmodels::auth::person_info::PersonInfoViewModel;
use identity_service::viewmodels::auth::update_user::UpdateUserViewModel;
use identity_service::viewmodels::auth::update_pwd::ChangePasswordViewModel;
use identity_service::viewmodels::auth::delete_user::DeleteUserViewModel;
use identity_service::viewmodels::auth::user_id::UserIdViewModel;
use identity_service::viewmodels::auth::flag::FlagHolder;
use identity_service::generic_token::GenericTokenViewModel;
use identity_service::service::mail_service::MailTransport;
use identity_service::map_token_pwd::TokenHolderForgottenPwd;
use crate::delegates;
use rocket::State;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![ 
        registration,
        login,
        return_new_token,
        get_profile,
        update_user,
        change_password,
        add_flag,
        remove_flag,
        delete_user,
        send_email_forgotten_pwd,
        change_forgotten_password
    ]
}

/**
 * Function used to add a user through help of the viewmodel RegistrationViewModel, if it succeeds it returns a normal json object and if there are errors a json object with errors is sent.
 */
#[post("/registration", format = "application/json", data = "<model>")]
fn registration(model : Json<RegistrationViewModel>, sled_db : State<StoreManager>, transport : State<MailTransport>) -> JsonValue {
    match person_service::add_user(model.0, &sled_db.give_unique_id(),sled_db.give_store(),Some(delegates::user_creation) ,&transport) {
        Ok(_) => {
            info!("A user has been added");
            json!({
                "ok" : true,
                "message" : "User has been added"
            })
        },
        Err(e) => error_controller::return_error_json(e, false)
    }
}

/**
 * Function used to control the credentials and return a token in the returned json object. When the credentials aren't valid a json object that indicate the error is returned.
 */
#[post("/login", format = "application/json", data = "<model>")]
fn login(model : Json<LoginViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::check_credentials(model.0,sled_db.give_store()) {
        Ok(claim_of_user) => {
            info!("The given credentials are right");
            json!({
                "ok" : true,
                "token" : claim_of_user.token_from_user().unwrap()
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Function used to give a new token after it controls the given token, if this token is okay then a new token will be sent.
 */
#[post("/token", format = "application/json", data = "<model>")]
fn return_new_token(model : Json<TokenHolderViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::get_new_token(model.0,sled_db.give_store()) {
        Ok(claim_of_user) => {
            info!("A new token has been given");
            json!({
                "ok" : true,
                "token" : claim_of_user.token_from_user().unwrap()
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Function used to update user throught the help of viewmodel UpdateUserViewModel, this one contains the token that after validation can be used to modify certain properties of the user. If the operations succeeds a normal json object is sent, if it doesn't a json object indicating an error is sent back.
 */
#[put("/update", format = "application/json", data = "<model>")]
fn update_user(model : Json<GenericTokenViewModel<UpdateUserViewModel>>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::update_user(model.0,sled_db.give_store()) {
        Ok(_) => {
            info!("The user has successfully been updated");
            json!({
                "ok" : true,
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Function used to return basic information about the user by validating the token within the viewmodel TokenHolderViewModel. The basic information of the user is returned in the json object, and if the token validation fails a json object returned with the error within.
 */
#[post("/profile", format = "application/json", data = "<model>")]
fn get_profile(model : Json<TokenHolderViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::check_token(model.0,sled_db.give_store()) {
        Ok(user) => {
            info!("Profile information has been send to the user");
            json!({
                "ok" : true,
                "person" : PersonInfoViewModel::from_identity_user(&user)
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Function used to change the password of an user. A function is used to control the token and control the password. If it succeeds a positive message passes, but if it fails a json object with the error within.
*/
#[put("/password", format = "application/json", data = "<model>")]
fn change_password(model : Json<GenericTokenViewModel<ChangePasswordViewModel>>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::change_password(model.0,sled_db.give_store()) {
        Ok(_) => {
            info!("The password of an user has been changed.");
            json!({
                "ok" : true,
                "message" : "User password has sucessfully been changed"
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

#[put("/flag/add", format = "application/json", data = "<model>")]
fn add_flag(model : Json<GenericTokenViewModel<FlagHolder>>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::add_flag_of_user(model.0,sled_db.give_store()) {
        Ok(_) => {
            info!("A flag has been added to the user.");
            json!({
                "ok" : true,
                "message" : "User has a flag added"
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

#[put("/flag/remove", format = "application/json", data = "<model>")]
fn remove_flag(model : Json<GenericTokenViewModel<FlagHolder>>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::remove_flag_of_user(model.0,sled_db.give_store()) {
        Ok(_) => {
            info!("A flag has been removed of the user.");
            json!({
                "ok" : true,
                "message" : "User removed the flag"
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Function used to delete an user, this will use the token to get the user id and to check  if this id exists or not and delete if it does. An error is thrown whent the token is empty or the user couldn't be deleted.
*/
#[delete("/delete", format = "application/json", data = "<model>")]
fn delete_user(model : Json<DeleteUserViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::delete_user(model.0,sled_db.give_store()) {
        Ok(_) => {
            info!("The user has been deleted");
            json!({
                "ok" : true,
                "message" : "User password has sucessfully been deleted"
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Function that is used to send an email to change the password of an user that has forgotten password. It will also store a token that will be used to authorize the change of the password.
 */
#[post("/forgotten_pwd", format = "application/json", data = "<model>")]
fn send_email_forgotten_pwd(model : Json<UserIdViewModel>, sled_db : State<StoreManager>, token_map_state : State<TokenHolderForgottenPwd>, transport : State<MailTransport>) -> JsonValue {
    match person_service::demand_email_changing_password(
        &token_map_state,
        model.0.get_id(),
        sled_db.give_store(),
        &transport,
        delegates::send_email_for_forgotten_pwd
    ) {
        Ok(_) => {
            info!("The user has succesfully demanded to change his password because he forgot it.");
            json!({
                "ok" : true,
                "message" : "Email with token to change forgotten password has been send."
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}

/**
 * Will take up the token out of the viewmodel and check it. If it is okay it will continue and pass through the change.
 */
#[post("/change_forgotten_pwd", format = "application/json", data = "<model>")]
fn change_forgotten_password(model : Json<ChangeForgottenPassword>, sled_db : State<StoreManager>, token_map_state : State<TokenHolderForgottenPwd>) -> JsonValue {
    match person_service::change_forgotten_password(
        &token_map_state,
        model.0,
        sled_db.give_store()
    ) {
        Ok(_) => {
            info!("The user has succesfully changed his password.");
            json!({
                "ok" : true,
                "message" : "Password has succesfully changed his password."
            })
        },
        Err(e) => error_controller::return_error_json(e,false)
    }
}