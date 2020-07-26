use crate::claim::Claim;
use crate::store::Store;
use crate::traits::token::TokenContainerTrait;
use crate::viewmodels::auth::delete_user::DeleteUserViewModel;
use crate::viewmodels::auth::login::LoginViewModel;
use crate::viewmodels::auth::registration::RegistrationViewModel;
use crate::viewmodels::auth::token::TokenHolderViewModel;
use crate::viewmodels::auth::update_pwd::ChangePasswordViewModel;
use crate::viewmodels::auth::update_user::UpdateUserViewModel;
use crate::viewmodels::auth::person_info::PersonInfoViewModel;
use identity_dal::traits::t_user::UserTrait;
use identity_dal::traits::t_user_manager::UserStoreTrait;
use identity_dal::user::identity_user::IdentityUser;

/**
 * Function used to add an user to the sled no-sql database. The viewmodel from which the user will be added will be controlled on the fact that the password and confirmed password need to equal each other or otherwhise an error will be returned. An error will also be thrown if it couldn't add a user to the store.
 */
pub fn add_user(
    model: RegistrationViewModel,
    id: &str,
    db: Store
) -> Result<IdentityUser, &'static str> {
    if model.get_confirmed_password() != model.get_password() {
        warn!("A password and its confirmation has to be the same");
        return Err("Password and confirmed password aren't the same.");
    }
    if db.is_email_taken(model.get_email()) {
        warn!("The email is already taken in the sled database");
        return Err("This email is already taken, please take another one.");
    }
    let person = match IdentityUser::new_user_with_personal_id(
        id,
        model.get_email(),
        "",
        "",
        model.get_password(),
    ) {
        Ok(user) => user,
        Err(e) => {
            error!("An user could not be made");
            return Err(e);
        }
    };
    match db.add_user(person) {
        Ok(user) => Ok(user),
        Err(_) => {
            error!("Could not add a user to the sled database");
            Err("Could not add a user")
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
    model: UpdateUserViewModel,
    db: Store
) -> Result<bool, &'static str> {
    let mut user = match Claim::token_to_user(&model.get_token(), &db) {
        Ok(user) => user,
        Err(e) => {
            error!("Could not map a jwt token to an user from the sled database");
            return Err(e);
        }
    };
    if let Some(new_email) = model.new_email {
        if !db.is_email_taken(&new_email) {
            user.email = new_email;
        }
    }
    if let Some(new_first_name) = model.new_first_name {
        user.first_name = new_first_name;
    }
    if let Some(new_last_name) = model.new_last_name {
        user.last_name = new_last_name;
    }
    Ok(db.update_user(&user.id, &user).expect("Could not update a user."))
}

/**
 * Method used to control credentials of an user. This returns a claim that can be used to be authorized as the user.
 *
 * An error is returned when the credentials are false and when the email is not found.
 */
pub fn check_credentials(model: LoginViewModel, db: Store) -> Result<Claim, &'static str> {
    if let Some(user) = db.get_user_by_email(model.get_email()) {
        if !user.check_pwd(model.get_password()) {
            warn!("The user's password is not good.");
            return Err("Password is not right");
        }
        return match Claim::new_claim(user.id.as_ref()) {
            Ok(claim) => Ok(claim),
            Err(e) => Err(e),
        };
    }
    warn!(
        "The email {} doesn't exist in the sled database",
        model.get_email()
    );
    Err("User doesn't exist, did not find the email.")
}

/**
 * Method used to check an token and to return the user associated with that token's subject.
 *
 * An error is returned when the sub property of the decoded token isn't found and when the token couldn't be decoded.
 */
pub fn check_token(token: TokenHolderViewModel, db: Store) -> Result<IdentityUser, &'static str> {
    Claim::token_to_user(token.get_token(), &db)
}

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
    model: ChangePasswordViewModel,
    db: Store,
) -> Result<bool, &'static str> {
    if model.get_token().is_empty() {
        return Err("A token can't be empty");
    }
    if model.get_password().is_empty() {
        return Err("A password can't be empty");
    }
    if model.get_password() != model.get_confirm_password() {
        return Err("Password and password confirmed aren't the same");
    }
    let mut user: IdentityUser = Claim::token_to_user(&model.get_token(), &db)?;
    match user.set_password(&model.get_password()) {
        Ok(_) => match db.update_user(&user.id, &user) {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

/**
 * Function used to delete a user, the viewmodel TokenHolderViewModel is used to check for authorization and to get the id of the user. The id of the user is used to check if he exists and if he exists he is deleted. An error is thrown if the token is false or if the person didn't exist.
*/
pub fn delete_user(model: DeleteUserViewModel, db: Store) -> Result<bool, &'static str> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if let Some(user) = db.get_user_by_uuid(&claim_token.claims.sub) {
        if !user.check_pwd(&model.get_password()) && !model.is_delete_confirmed() {
            warn!("The user's password or delete confirmation was not good, the user could not be deleted");
            return Err("The user's password or delete confirmation was not good")
        }
        info!("User password and password confirmation was good and user is going to be deleted.");
        return Ok(db.delete_user(&user.id) .expect("The deletion of the user didn't succeed."))
    } 
    warn!("Can't delete a user if he doesn't exist");
    Err("User doesn't exist")
}