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
use identity_dal::traits::t_admin_manager::AdminStoreTrait;

/**
 * Function that the admin is used to create an user with its personal email, password and id.
 * 
 * Throws an error when:
 * * the password and its confirmation aren't the same
 * * if the user's email already is taken
 * * When the id from the token is not the right one, that of an admin
 */
pub fn create_user(
    model: AdminCreateUserViewModel,
    id: &str,
    db: Store
) -> Result<IdentityUser, &'static str> {
    if model.get_confirmed_password() != model.get_password() {
        warn!("A password and its confirmation has to be the same");
        return Err("Password and confirmed password aren't the same.")
    }
    if db.is_email_taken(model.get_email()) {
        warn!("The email is already taken in the sled database");
        return Err("This email is already taken, please take another one.")
    }
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        let person = match IdentityUser::new_user_with_personal_id(id,model.get_email(),"","",model.get_password()) {
            Ok(user) => user,
            Err(e) => {
                error!("An user could not be made");
                return Err(e);
            }
        };
        return match db.add_user(person) {
            Ok(user) => Ok(user),
            Err(_) => {
                error!("Could not add a user to the sled database");
                Err("Could not add a user")
            }
        }
    }
    warn!("The user id wasn't that of an admin. id: {}",&claim_token.claims.sub);
    Err("The user id wasn't that of an admin.")
}

/**
 * Controls the id of an token so that it is equal to that of an admin. The user id that comes in the viewmodel is used to delete the user.
 */
pub fn delete_user(
    model : DeleteUserViewModel,
    db : Store
) -> Result<bool,&'static str> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        return Ok(db.delete_user(model.get_user_id()).expect("The deletion of the user didn't succeed."))
    } 
    warn!("Token user id isn't that one of the admin");
    Err("User id isn't that one of the admin")
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
    model : AdminUpdateUserViewModel,
    db : Store
) -> Result<bool,&'static str> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        let mut user = db.get_user_by_uuid(model.get_user_id())
            .expect("Could not map the user id to an actual user in the sled database.");
        if let Some(new_email) = &model.new_email {
            if !db.is_email_taken(&new_email) {
                user.email = new_email.clone();
            }
        }
        if let Some(new_first_name) = &model.new_first_name {
            user.first_name = new_first_name.clone();
        }
        if let Some(new_last_name) = &model.new_last_name {
            user.last_name = new_last_name.clone();
        }
        return Ok(db.update_user(model.get_user_id(), &user).expect("Could not update a user."))
    } 
    warn!("Token user id isn't that one of the admin");
    Err("User id isn't that one of the admin")
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
    model : AdminChangePasswordUserViewModel,
    db : Store
) -> Result<bool,&'static str> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        if model.get_password().is_empty() {
            return Err("A password can't be empty");
        }
        if model.get_password() != model.get_confirm_password() {
            return Err("Password and password confirmed aren't the same");
        }
        let mut user = db.get_user_by_uuid(model.get_id_user())
            .expect("Could not map the user id to an actual user in the sled database.");
        return match user.set_password(&model.get_password()) {
            Ok(_) => db.update_user(&user.id, &user),
            Err(e) => Err(e),
        }
    }
    warn!("Token user id isn't that one of the admin");
    Err("User id isn't that one of the admin")
}

/**
 * Returns a result with a collection of all non admin users, can only be called through a admin user.
 */
pub fn get_all_users(
    model : TokenHolderViewModel,
    db : Store
) -> Result<AllNonAdminUsersViewModel,&'static str> {
    let claim_token = Claim::decode_token_viewmodel(&model)?;
    if db.is_id_admin(&claim_token.claims.sub) {
        return Ok(AllNonAdminUsersViewModel::from_users_vector(db.get_non_admin_users()))
    }
    warn!("Token user id isn't that one of the admin");
    Err("User id isn't that one of the admin")
}