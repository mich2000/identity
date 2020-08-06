use rocket_contrib::json::{Json,JsonValue};
use super::error_controller;
use identity_service::service::person_service;
use identity_service::store::StoreManager;
use identity_service::viewmodels::auth::registration::RegistrationViewModel;
use identity_service::viewmodels::auth::login::LoginViewModel;
use identity_service::viewmodels::auth::token::TokenHolderViewModel;
use identity_service::viewmodels::auth::person_info::PersonInfoViewModel;
use identity_service::viewmodels::auth::update_user::UpdateUserViewModel;
use identity_service::viewmodels::auth::update_pwd::ChangePasswordViewModel;
use identity_service::viewmodels::auth::delete_user::DeleteUserViewModel;
use rocket::State;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![registration,login,get_profile,update_user,change_password,delete_user]
}

/**
 * Function used to add a user through help of the viewmodel RegistrationViewModel, if it succeeds it returns a normal json object and if there are errors a json object with errors is sent.
 */
#[post("/registration", format = "application/json", data = "<json_registration>")]
fn registration(json_registration : Json<RegistrationViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::add_user(json_registration.0, &sled_db.give_unique_id(),sled_db.give_store(),
    &sled_db.give_user_creation_fun()) {
        Ok(_) => {
            info!("A user has been added");
            json!({
                "Status" : "OK",
                "Message" : "User has been added"
            })
        },
        Err(e) => error_controller::return_error_json(e)
    }
}

/**
 * Function used to control the credentials and return a token in the returned json object. When the credentials aren't valid a json object that indicate the error is returned.
 */
#[post("/login", format = "application/json", data = "<json_login>")]
fn login(json_login : Json<LoginViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::check_credentials(json_login.0,sled_db.give_store()) {
        Ok(claim_of_user) => {
            info!("The given credentials are right");
            json!({
                "Status" : "Ok",
                "token" : claim_of_user.token_from_user().unwrap()
            })
        },
        Err(e) => error_controller::return_error_json(e)
    }
}

/**
 * Function used to update user throught the help of viewmodel UpdateUserViewModel, this one contains the token that after validation can be used to modify certain properties of the user. If the operations succeeds a normal json object is sent, if it doesn't a json object indicating an error is sent back.
 */
    #[put("/update", format = "application/json", data = "<update_user_viewmodel>")]
    fn update_user(update_user_viewmodel : Json<UpdateUserViewModel>, sled_db : State<StoreManager>) -> JsonValue {
        match person_service::update_user(update_user_viewmodel.0,sled_db.give_store()) {
            Ok(_) => {
                info!("The user has successfully been updated");
                json!({
                    "Status" : "Ok",
                })
            },
            Err(e) => error_controller::return_error_json(e)
        }
    }

/**
 * Function used to return basic information about the user by validating the token within the viewmodel TokenHolderViewModel. The basic information of the user is returned in the json object, and if the token validation fails a json object returned with the error within.
 */
#[post("/profile", format = "application/json", data = "<token>")]
fn get_profile(token : Json<TokenHolderViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::check_token(token.0,sled_db.give_store()) {
        Ok(user) => {
            info!("Profile information has been send to the user");
            json!({
                "Status" : "Ok",
                "person" : PersonInfoViewModel::from_identity_user(&user)
            })
        },
        Err(e) => error_controller::return_error_json(e)
    }
}

/**
 * Function used to change the password of an user. A function is used to control the token and control the password. If it succeeds a positive message passes, but if it fails a json object with the error within.
*/
#[put("/password", format = "application/json", data = "<model>")]
fn change_password(model : Json<ChangePasswordViewModel>, sled_db : State<StoreManager>) -> JsonValue {
    match person_service::change_password(model.0,sled_db.give_store()) {
        Ok(_) => {
            info!("The password of an user has been changed.");
            json!({
                "Status" : "Ok",
                "Message" : "User password has sucessfully been changed"
            })
        },
        Err(e) => error_controller::return_error_json(e)
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
                "Status" : "Ok",
                "Message" : "User password has sucessfully been deleted"
            })
        },
        Err(e) => error_controller::return_error_json(e)
    }
}