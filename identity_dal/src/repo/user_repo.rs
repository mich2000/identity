use crate::traits::t_user_manager::UserStoreTrait;
use crate::traits::t_user::UserTrait;
use crate::traits::t_admin_manager::AdminStoreTrait;
use crate::user::identity_user::IdentityUser;
use sled::Tree;
use super::user_config::UserConfig;
use crate::user::identity_user;
use crate::user::identity_user::RESERVED_ID;
use crate::err::IdentityError;

/**
 * User store represents a tree within a NO-SQL sled database, this will be the object through which user data will be solved.
 */
#[derive(Clone)]
pub struct UserStore {
    pub user_db_tree : Tree
}

impl UserStore {
    /**
     * Return a new tree on a database. The tree is opened on a sled database through a given path and the tree name. If the path and tree are empty then a temporary database is created in memory.
     */
    pub fn new_db(config : UserConfig) -> UserStore {
        match config.get_db().open_tree(&config.get_tree()) {
            Ok(tree) => UserStore{ user_db_tree : tree },
            Err(_) => panic!("Could not open the tree {}", &config.get_tree())
        }
    }
}

impl UserStoreTrait<IdentityUser> for UserStore {
    /**
     * This is the setup method, that is used to control if the admin is in the sled and to insert him if he isn't there. Function is also used to flush dirty buffers before controlling if the admin is in the database.
    */
    fn setup(&self) -> Result<(),IdentityError> {
        if !self.is_id_taken(&identity_user::RESERVED_ID) {
            info!("The admin will be added because he was not present before.");
            match self.add_user(IdentityUser::admin().unwrap()) {
                Ok(_) => (),
                Err(_) => return Err(IdentityError::UserCannotBeAdded)
            }
        }
        info!("Admin is present");
        Ok(())
    }

    /**
     * Adds an user to the sled database based on the email and password of an user, the id of the added user can be given this time.
     * 
     * Errors can be thrown because:
     * email or password are empty
     * email isn't in a valid format or taken
     * id of the user is already taken
     */
    fn create_user_with_personal_id(&self, id : &str, email : &str, pwd : &str) -> Result<IdentityUser, IdentityError> {
        if email.is_empty() && pwd.is_empty() {
            return Err(IdentityError::EmailAndPasswordIsEmpty)
        }
        if !crate::util::control_email(email) {
            return Err(IdentityError::EmailNotCorrectFormat)
        }
        if self.is_email_taken(&email) {
            return Err(IdentityError::EmailIsAlreadyTaken)
        }
        let ps : IdentityUser = IdentityUser::new_user_with_personal_id(id ,email, "", pwd).unwrap();
        if self.is_id_taken(ps.get_id()) {
            return Err(IdentityError::IdIsAlreadyTaken)
        }
        self.user_db_tree.insert(ps.get_id(), &ps).unwrap();
        Ok(ps)
    }
    
    /**
     * Adds an user to the sled database based on the email and password of an user.
     * 
     * Errors can be thrown because:
     * email or password are empty
     * email isn't in a valid format or taken
     * id of the user is already taken
     */
    fn create_user(&self, email : &str, pwd : &str) -> Result<IdentityUser, IdentityError> {
        if email.is_empty() && pwd.is_empty() {
            return Err(IdentityError::EmailAndPasswordIsEmpty)
        }
        if !crate::util::control_email(email) {
            return Err(IdentityError::EmailIsAlreadyTaken)
        }
        if self.is_email_taken(&email) {
            return Err(IdentityError::EmailIsAlreadyTaken)
        }
        let ps : IdentityUser = IdentityUser::new_user(email,"", pwd).unwrap();
        if self.is_id_taken(ps.get_id()) {
            return Err(IdentityError::IdIsAlreadyTaken)
        }
        match self.user_db_tree.insert(ps.get_id(), &ps) {
            Ok(_) => Ok(ps),
            Err(_) => Err(IdentityError::UserAlreadyPresent)
        }
    }

    /**
     * Adds an user to the sled database.
     * 
     * Errors can be thrown because:
     * email or password are empty
     * email isn't in a valid format or taken
     * id of the user is already taken
     */
    fn add_user(&self, user : IdentityUser) -> Result<IdentityUser, IdentityError> {
        if user.get_email().is_empty() && user.is_pwd_empty() {
            return Err(IdentityError::EmailAndPasswordIsEmpty)
        }
        if self.is_id_taken(user.get_id()) {
            return Err(IdentityError::IdIsAlreadyTaken)
        }
        if !crate::util::control_email(user.get_email()) {
            return Err(IdentityError::EmailNotCorrectFormat)
        }
        if self.is_email_taken(&user.get_email()) {
            return Err(IdentityError::UserCannotBeAdded)
        }
        self.user_db_tree.insert(user.get_id(), &user).unwrap();
        Ok(user)
    }

    /**
     * Returns a bool saying if an email is already taken in the database.
     */
    fn is_email_taken(&self,email : &str) -> bool {
        self.user_db_tree
            .iter()
            .any(|ps| IdentityUser::from(&ps.unwrap().1).get_email() == email)
    }

    /**
     * Returns a bool indicating if a id has been taken
     */
    fn is_id_taken(&self, id : &str) -> bool {
        match self.user_db_tree.contains_key(id) {
            Ok(result) => result,
            Err(_) => false
        }
    }
    
    /**
     * Returns a user which has a specific email, if none has this email a None is given.
     */
    fn get_user_by_email(&self, email : &str) -> Option<IdentityUser> {
        self.user_db_tree
        .iter()
        .map(| ps | IdentityUser::from(&ps.unwrap().1))
        .find(| ps | ps.get_email() == email)
    }
    
    /**
     * Returns a user based on a key or his id, if none has the key the a None is returned.
     */
    fn get_user_by_uuid(&self, uuid : &str) -> Option<IdentityUser> {
        match self.user_db_tree.get(uuid).unwrap() {
            Some(user) => Some(IdentityUser::from(&user)),
            None => None
        }
    }

    /**
     * Updates an user based on his id or key in the sled database. If the update is successfull it will return a boolean and if the id of the user can't be found an error will be returned.
     */
    fn update_user(&self, id : &str, user : &IdentityUser) -> Result<bool, IdentityError> {
        if let Some(mut old_user) = self.get_user_by_uuid(id) {
            old_user.set_email(user.get_email()).expect("Could not change the email of the user.");
            old_user.set_user_name(user.get_user_name());
            old_user.set_hashed_password(user.get_hashed_password());
            old_user.set_security_stamp(user.get_security_stamp());
            old_user.set_flags(user.get_flags());
            return Ok(
                self.user_db_tree.insert(
                    &id,
                    &old_user)
                    .is_ok()
            );
        }
        Err(IdentityError::UserIsNotPresent)
    }
    
    /**
     * Deletes an user based on its id.
     * 
     * An error is thrown when the id is nothing or when its equal to the admin id.
     */
    fn delete_user(&self, id : &str) -> Result<bool, IdentityError> {
        if id.is_empty() {
            return Err(IdentityError::IdIsAlreadyTaken)
        }
        if id == RESERVED_ID {
            return Err(IdentityError::IdEqualsAdmin)
        }
        Ok(self.user_db_tree.remove(id).unwrap().is_some())
    }
    
    /**
     * Returns a boolean based on the comparison between the user's password and given parameter password.
     * 
     * Errors can be thrown because:
     * password is empty
     * email is not in a valid format
     * person is equal to nothing
     */
    fn check_user_password(&self, email : &str, pwd : &str) -> Result<bool, IdentityError> {
        if pwd.is_empty() {
            return Err(IdentityError::PasswordIsEmpty)
        }
        if !crate::util::control_email(email) {
            return Err(IdentityError::EmailNotCorrectFormat)
        }
        let person = self.get_user_by_email(email);
        Ok(person.ok_or("Person doesn't exist").unwrap().check_pwd(pwd))
    }
}

impl AdminStoreTrait<IdentityUser> for UserStore {
    /**
     * Returns the amount of non admin users in the sled database.
     */
    fn get_amount_of_non_admin_users(&self) -> usize {
        self.user_db_tree.len() - (self.is_id_taken(RESERVED_ID) as usize)
    }

    /**
     * Controls if the given id is of the admin
     */
    fn is_id_admin(&self,id : &str) -> bool {
        id == RESERVED_ID
    }

    /**
     * Returns the admin IdentityUser
     */
    fn get_admin(&self) -> Result<IdentityUser,IdentityError> {
        self.get_user_by_uuid(RESERVED_ID).ok_or(IdentityError::AdminNotPresent)
    }

    /**
     * Returns a collection of all identity users that aren't admins
     */
    fn get_non_admin_users(&self) -> Vec<IdentityUser> {
        self.user_db_tree.iter()
        .map(|ps| IdentityUser::from(&ps.unwrap().1))
        .filter(|ps| ps.get_id() != RESERVED_ID)
        .collect()
    }
}

#[test]
fn test_update() {
    let db = UserStore::new_db(UserConfig::new_config("","",100000));
    
    let mut ps = db.add_user(IdentityUser::new_user("michael@outlook.be","","hertsens").unwrap()).unwrap();
    assert_eq!(ps.get_email(),"michael@outlook.be");
    assert!(db.check_user_password("michael@outlook.be", "hertsens").unwrap());

    ps.set_email("michael@michael.be").unwrap();
    ps.set_password("michael@michael.be").unwrap();
    db.update_user(ps.get_id(), &ps.to_owned()).unwrap();
    ps = db.get_user_by_email("michael@michael.be").unwrap();

    assert_eq!(ps.get_email(),"michael@michael.be");
    assert!(db.check_user_password("michael@michael.be", "michael@michael.be").unwrap());
}