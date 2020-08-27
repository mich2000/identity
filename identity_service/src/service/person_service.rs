use crate::claim::Claim;
use crate::store::Store;
use crate::store::UserDelegate;
use crate::viewmodels::auth::delete_user::DeleteUserViewModel;
use crate::viewmodels::auth::login::LoginViewModel;
use crate::viewmodels::auth::registration::RegistrationViewModel;
use crate::viewmodels::auth::token::TokenHolderViewModel;
use crate::viewmodels::auth::update_pwd::ChangePasswordViewModel;
use crate::viewmodels::auth::update_user::UpdateUserViewModel;
use crate::viewmodels::auth::person_info::PersonInfoViewModel;
use crate::viewmodels::auth::change_pwd::ChangeForgottenPassword;
use crate::viewmodels::auth::flag::FlagHolder;
use crate::map_token_pwd::HashMapTokenPasswordChange;
use identity_dal::traits::t_user::UserTrait;
use identity_dal::traits::t_user_manager::UserStoreTrait;
use identity_dal::user::identity_user::IdentityUser;
use identity_dal::err::IdentityError;
use std::sync::Mutex;
use crate::service::mail_service::MailTransport;

/**
 * Function used to add an user to the sled no-sql database. The viewmodel from which the user will be added will be controlled on the fact that the password and confirmed password need to equal each other or otherwhise an error will be returned. An error will also be thrown if it couldn't add a user to the store.
 */
pub fn add_user(
    model: RegistrationViewModel,
    id: &str,
    db: Store,
    user_creation_function : UserDelegate,
    transport : &MailTransport
) -> Result<IdentityUser, IdentityError> {
    if model.get_confirmed_password() != model.get_password() {
        warn!("A password and its confirmation has to be the same");
        return Err(IdentityError::PasswordAndPasswordConfirmedNotEqual)
    }
    if db.is_email_taken(model.get_email()) {
        warn!("The email is already taken in the sled database");
        return Err(IdentityError::EmailIsAlreadyTaken);
    }
    let person = match IdentityUser::new_user_with_personal_id(
        id,
        model.get_email(),
        "",
        model.get_password()) {
        Ok(user) => user,
        Err(e) => {
            error!("An user could not be made");
            return Err(e)
        }
    };
    match db.add_user(person) {
        Ok(user) => {
            if let Some(fun) = user_creation_function {
                match fun(&user.get_id(),&db, &transport) {
                    Ok(_) => info!("User creation function has succesfully been executed."),
                    Err(e) => warn!("User creation function has failed. Error: {}",e)
                }
            }
            Ok(user)
        },
        Err(_) => {
            error!("Could not add a user to the sled database");
            Err(IdentityError::UserCannotBeAdded)
        }
    }
}

/**
 * Function to update a user based on a token and information to update it. In the model that is passed through it contains a token which when validated offers the possbility to the user.
 *
 * Attributes that when not empty in the model, updates the user:
 * * new_email : updates the email of the user
 * * new_first_name : updates the first name of the user
 * * new_last_name : updates the last name of the user
 **/
pub fn update_user(
    token : &str,
    model: UpdateUserViewModel,
    db: Store
) -> Result<bool, IdentityError> {
    let mut user = match Claim::token_to_user(token, &db) {
        Ok(user) => user,
        Err(e) => {
            error!("Could not map a jwt token to an user from the sled database");
            return Err(e);
        }
    };
    if let Some(new_email) = &model.new_email {
        if !db.is_email_taken(&new_email) {
            user.set_email(&new_email).expect("Could not change the email of the user.");
        }
    }
    if let Some(new_user_name) = &model.new_user_name {
        user.set_user_name(&new_user_name);
    }
    Ok(db.update_user(user.get_id(), &user).expect("Could not update a user."))
}

/**
 * Method used to control credentials of an user. This returns a claim that can be used to be authorized as the user.
 *
 * An error is returned when the credentials are false and when the email is not found.
 */
pub fn check_credentials(model: LoginViewModel, db: Store) -> Result<Claim, IdentityError> {
    if let Some(user) = db.get_user_by_email(model.get_email()) {
        if !user.check_pwd(model.get_password()) {
            warn!("The user's password is not good.");
            return Err(IdentityError::PasswordIsNotCorrect);
        }
        return match Claim::new_read_write_claim(user.get_id()) {
            Ok(claim) => Ok(claim),
            Err(e) => Err(e),
        };
    }
    warn!(
        "The email {} doesn't exist in the sled database",
        model.get_email()
    );
    Err(IdentityError::UserIsNotPresent)
}

/**
 * Method used to check an token and to return the user associated with that token's subject.
 *
 * An error is returned when the sub property of the decoded token isn't found and when the token couldn't be decoded.
 */
pub fn check_token(token : &str, db: Store) -> Result<IdentityUser, IdentityError> {
    Claim::token_to_user(token, &db)
}

/**
 * Takes in a viewmodel which has a token, which is then controlled and checked.
 * 
 * A Claim is then send back.
 */
pub fn get_new_token(token: TokenHolderViewModel, db: Store) -> Result<Claim, IdentityError> {
    match Claim::decode_token(token.get_token()) {
        Ok(claim) => {
            if db.is_id_taken(&claim.claims.sub) {
                return Ok(Claim::new_read_write_claim(&claim.claims.sub)?)
            }
            Err(IdentityError::UserIsNotPresent)
        },
        Err(e) => Err(e)
    }
}

/**
 * Method used to get a viewmodel PersonInfoViewModel which contains a basic person info about this user.
 */
pub fn get_user_info(id : &str, db : &Store) -> Option<PersonInfoViewModel> {
    match db.get_user_by_uuid(id) {
        Some(user) => Some(PersonInfoViewModel::from_identity_user(&user)) ,
        None => None
    }
}

/**
 * Method that is used to change the user's password through the help of the viewmodel ChangePasswordViewModel.
 *
 * An error is thrown when:
 * * token is empty
 * * password and password confirm aren't the same
 */
pub fn change_password(
    token : &str,
    model: ChangePasswordViewModel,
    db: Store,
) -> Result<bool, IdentityError> {
    if token.is_empty() {
        return Err(IdentityError::TokenIsEmpty)
    }
    if model.get_password().is_empty() {
        return Err(IdentityError::PasswordIsEmpty)
    }
    if model.get_password() != model.get_confirm_password() {
        return Err(IdentityError::PasswordAndPasswordConfirmedNotEqual)
    }
    let mut user: IdentityUser = Claim::token_to_user(token,&db)?;
    match user.set_password(&model.get_password()) {
        Ok(_) => match db.update_user(user.get_id(), &user) {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

/**
 * Function used to delete a user, the viewmodel TokenHolderViewModel is used to check for authorization and to get the id of the user. The id of the user is used to check if he exists and if he exists he is deleted. An error is thrown if the token is false or if the person didn't exist.
*/
pub fn delete_user(token : &str,model: DeleteUserViewModel, db: Store) -> Result<bool, IdentityError> {
    let claim_token = Claim::decode_token(token)?;
    if let Some(user) = db.get_user_by_uuid(&claim_token.claims.sub) {
        if !user.check_pwd(&model.get_password()) && !model.is_delete_confirmed() {
            warn!("The user's password or delete confirmation was not good, the user could not be deleted");
            return Err(IdentityError::UserDeleteFailed)
        }
        info!("User password and password confirmation was good and user is going to be deleted.");
        return match db.delete_user(user.get_id()) {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }
    warn!("Can't delete a user if he doesn't exist");
    Err(IdentityError::UserIsNotPresent)
}

pub fn add_flag_of_user(
    token : &str,
    model: FlagHolder,
    db: Store,
) -> Result<bool, IdentityError> {
    if model.get_flag().is_empty() {
        return Err(IdentityError::CustomError("A flag cannot be empty".to_owned()))
    }
    let mut user: IdentityUser = Claim::token_to_user(token,&db)?;
    user.add_flag(&model.get_flag());
    match db.update_user(&user.get_id(), &user) {
        Ok(result) => {
            info!("flag {} has been added",&model.get_flag());
            Ok(result)
        },
        Err(e) => Err(e)
    }
}

pub fn remove_flag_of_user(
    token : &str,
    model: FlagHolder,
    db: Store,
) -> Result<bool, IdentityError> {
    if model.get_flag().is_empty() {
        return Err(IdentityError::CustomError("A flag cannot be empty".to_owned()))
    }
    let mut user: IdentityUser = Claim::token_to_user(token,&db)?;
    user.remove_flag(&model.get_flag());
    match db.update_user(&user.get_id(), &user) {
        Ok(result) => {
            info!("flag {} has been removed",&model.get_flag());
            Ok(result)
        },
        Err(e) => Err(e)
    }
}

/**
 * Function that is used to insert a token into the token map, through the given user id. If the token has been inserted then a function that was given as a parameter will be executed that is responsible for sending the email so the user can change his password.
 */
pub fn demand_email_changing_password(
    token_map : &Mutex<HashMapTokenPasswordChange>,
    user_id : &str,
    store : Store,
    transport : &MailTransport,
    email_changing_function : fn(token : &str,store : &Store, token_map : &Mutex<HashMapTokenPasswordChange>, transport : &MailTransport) -> Result<(), IdentityError>
) -> Result<(),IdentityError> {
    let token_locked_map = &mut token_map.lock()
    .map_err(|_| IdentityError::CustomError("Could not lock the token map which gaurds tokens for changing password".to_owned()))?;
    if let Some(token) = token_locked_map.insert_new_user_request(user_id) {
        match email_changing_function(&token,&store, &token_map, transport) {
            Ok(_) => info!("Managed to execute the function responsible to manage sending the email so user can change his password."),
            Err(_) => warn!("Could not execute the functions responsible to manage sending the email so user can change his password.")
        }
    }
    Ok(())
}

pub fn change_forgotten_password(
    token_map : &Mutex<HashMapTokenPasswordChange>,
    token : ChangeForgottenPassword,
    store : Store,
) -> Result<(),IdentityError> {
    let token_locked_map = &mut token_map.lock()
    .map_err(|_| IdentityError::CustomError("Could not lock the token map which gaurds tokens for changing password".to_owned()))?;
    if !token_locked_map.is_token_okay(token.get_token_forgotten_pwd()) {
        return Err(IdentityError::CustomError("Could not locate the token needed for changing password".to_owned()))
    }
    if token.get_confirm_password() != token.get_password() {
        return Err(IdentityError::PasswordAndPasswordConfirmedNotEqual)
    }
    if let Some(user_id) = token_locked_map.get_user_id_from_token(token.get_token_forgotten_pwd()) {
        if let Some(mut user) = store.get_user_by_uuid(&user_id) {
            user.set_password(token.get_password())?;
            if !store.update_user(&user_id , &user)? {
                return Err(IdentityError::UserCannotBeUpdated)
            }
        }
    }
    Ok(())
}