use crate::claim::Claim;
use crate::store::Store;
use identity_dal::traits::t_user::UserTrait;
use identity_dal::user::identity_user::IdentityUser;
use identity_dal::traits::t_user_manager::UserStoreTrait;
use crate::viewmodels::admin::create_user::AdminCreateUserViewModel;
use crate::viewmodels::admin::delete_user::DeleteUserViewModel;
use crate::viewmodels::admin::update_user::AdminUpdateUserViewModel;
use crate::viewmodels::admin::update_user_pwd::AdminChangePasswordUserViewModel;
use crate::viewmodels::auth::token::TokenHolderViewModel;
use crate::viewmodels::admin::all_users::AllNonAdminUsersViewModel;
use crate::generic_token::GenericTokenViewModel;
use identity_dal::traits::t_admin_manager::AdminStoreTrait;
use crate::IdentityError;

/**
 * Function that the admin is used to create an user with its personal email, password and id.
 * 
 * Throws an error when:
 * * the password and its confirmation aren't the same
 * * if the user's email already is taken
 * * When the id from the token is not the right one, that of an admin
 */
pub fn create_user(
    model: GenericTokenViewModel<AdminCreateUserViewModel>,
    id: &str,
    db: Store
) -> Result<IdentityUser, IdentityError> {
    if model.get_model().get_confirmed_password() != model.get_model().get_password() {
        warn!("A password and its confirmation has to be the same");
        return Err(IdentityError::PasswordAndPasswordConfirmedNotEqual)
    }
    if db.is_email_taken(model.get_model().get_email()) {
        warn!("The email is already taken in the sled database");
        return Err(IdentityError::EmailIsAlreadyTaken)
    }
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        let person = match IdentityUser::new_user_with_personal_id(id,model.get_model().get_email(),"","",model.get_model().get_password()) {
            Ok(user) => user,
            Err(e) => {
                error!("An user could not be made");
                return Err(IdentityError::CustomError(format!("{}",e)))
            }
        };
        return match db.add_user(person) {
            Ok(user) => Ok(user),
            Err(_) => {
                error!("Could not add a user to the sled database");
                Err(IdentityError::UserCannotBeAdded)
            }
        }
    }
    warn!("The user id wasn't that of an admin. id: {}",&claim_token.claims.sub);
    Err(IdentityError::IdEqualsAdmin)
}

/**
 * Controls the id of an token so that it is equal to that of an admin. The user id that comes in the viewmodel is used to delete the user.
 */
pub fn delete_user(
    model : GenericTokenViewModel<DeleteUserViewModel>,
    db : Store
) -> Result<bool,IdentityError> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        return Ok(db.delete_user(model.get_model().get_user_id()).expect("The deletion of the user didn't succeed."))
    } 
    warn!("Token user id isn't that one of the admin");
    Err(IdentityError::IdNotEqualToAdmin)
}

/**
 * Controls the id of an token so that it is equal to that of an admin. It will then seek the id of the user if it exists and update this user on the attributes that aren't empty in the viewmodel(new_email, new_first_name,new_last_name).
 * 
 * An error is thrown when:
 * * id isn't that of one of the admin
 * * token is empty
 * * password and confirmation pasword aren't the same
 * * user id isn't mapped to an user
*/
pub fn update_user(
    model : GenericTokenViewModel<AdminUpdateUserViewModel>,
    db : Store
) -> Result<bool,IdentityError> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        let mut user = db.get_user_by_uuid(model.get_model().get_user_id())
            .expect("Could not map the user id to an actual user in the sled database.");
        if let Some(new_email) = &model.get_model().new_email {
            if !db.is_email_taken(&new_email) {
                user.set_email(new_email).expect("Could not change the email of the user.");
            }
        }
        if let Some(new_first_name) = &model.get_model().new_first_name {
            user.set_first_name(new_first_name);
        }
        if let Some(new_last_name) = &model.get_model().new_last_name {
            user.set_last_name(new_last_name);
        }
        return Ok(db.update_user(model.get_model().get_user_id(), &user).expect("Could not update a user."))
    } 
    warn!("Token user id isn't that one of the admin");
    Err(IdentityError::IdNotEqualToAdmin)
}

/**
 * Controls the id of an token so that it is equal to that of an admin. It will then seek the id of the user if it exists and update the user's password.
 * 
 * An error is thrown when:
 * * id isn't that of one of the admin
 * * token is empty
 * * password and confirmation pasword aren't the same
 * * user id isn't mapped to an user
 */
pub fn update_user_pwd(
    model : GenericTokenViewModel<AdminChangePasswordUserViewModel>,
    db : Store
) -> Result<bool,IdentityError> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        if model.get_model().get_password().is_empty() {
            return Err(IdentityError::PasswordIsEmpty)
        }
        if model.get_model().get_password() != model.get_model().get_confirm_password() {
            return Err(IdentityError::PasswordAndPasswordConfirmedNotEqual)
        }
        let mut user = db.get_user_by_uuid(model.get_model().get_id_user())
            .expect("Could not map the user id to an actual user in the sled database.");
        return match user.set_password(&model.get_model().get_password()) {
            Ok(_) => db.update_user(user.get_id(), &user),
            Err(e) => Err(IdentityError::CustomError(format!("{}",e))),
        }
    }
    warn!("Token user id isn't that one of the admin");
    Err(IdentityError::IdNotEqualToAdmin)
}

/**
 * Returns a result with a collection of all non admin users, can only be called through a admin user.
 */
pub fn get_all_users(
    model : TokenHolderViewModel,
    db : Store
) -> Result<AllNonAdminUsersViewModel,IdentityError> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        return Ok(AllNonAdminUsersViewModel::from_users_vector(db.get_non_admin_users()))
    }
    warn!("Token user id isn't that one of the admin");
    Err(IdentityError::IdNotEqualToAdmin)
}