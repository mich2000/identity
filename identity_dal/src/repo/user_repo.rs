use crate::traits::t_user_manager::UserStoreTrait;
use crate::traits::t_user::UserTrait;
use crate::traits::t_admin_manager::AdminStoreTrait;
use crate::user::identity_user::IdentityUser;
use sled::Tree;
use super::user_config::UserConfig;
use crate::user::identity_user;
use crate::user::identity_user::RESERVED_ID;

/**
 * User store represents a tree within a NO-SQL sled database, this will be the object through which user data will be solved.
 */
#[derive(Clone)]
pub struct UserStore {
    pub user_db_tree : Tree,
    pub user_created : Option<fn(id : &str) -> Result<(),&'static str>>
}

impl UserStore {
    /**
     * Return a new tree on a database. The tree is opened on a sled database through a given path and the tree name. If the path and tree are empty then a temporary database is created in memory.
     */
    pub fn new_db(config : UserConfig, user_creation : Option<fn(id : &str) -> Result<(),&'static str>>) -> UserStore {
        match config.get_db().open_tree(&config.get_tree()) {
            Ok(tree) => UserStore{ user_db_tree : tree, user_created : user_creation },
            Err(_) => panic!("Could not open the tree {}", &config.get_tree())
        }
    }
}

impl UserStoreTrait<IdentityUser> for UserStore {
    /**
     * This is the setup method, that is used to control if the admin is in the sled and to insert him if he isn't there. Function is also used to flush dirty buffers before controlling if the admin is in the database.
    */
    fn setup(&self) -> Result<(),&'static str> {
        if !self.is_id_taken(&identity_user::RESERVED_ID) {
            info!("The admin will be added because he was not present before.");
            self.add_user(IdentityUser::admin()?).expect("Could not insert ");
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
    fn create_user_with_personal_id(&self, id : &str, email : &str, pwd : &str) -> Result<IdentityUser, &'static str> {
        if email.is_empty() && pwd.is_empty() {
            return Err("Email and password can't be equal to nothing.")
        }
        if !validator::validate_email(email) {
            return Err("Email is not in a good format")
        }
        if self.is_email_taken(&email) {
            return Err("User already exists, this can't be added");
        }
        let ps : IdentityUser = IdentityUser::new_user_with_personal_id(id ,email,"", "", pwd).unwrap();
        if self.is_id_taken(&ps.id) {
            return Err("User ID has already taken")
        }
        self.user_db_tree.insert(&ps.id, bincode::serialize(&ps).unwrap().to_vec()).unwrap();
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
    fn create_user(&self, email : &str, pwd : &str) -> Result<IdentityUser, &'static str> {
        if email.is_empty() && pwd.is_empty() {
            return Err("Email and password can't be equal to nothing.")
        }
        if !validator::validate_email(email) {
            return Err("Email is not in a good format")
        }
        if self.is_email_taken(&email) {
            return Err("User already exists, this can't be added");
        }
        let ps : IdentityUser = IdentityUser::new_user(email,"", "", pwd).unwrap();
        if self.is_id_taken(&ps.id) {
            return Err("User ID has already taken")
        }
        match self.user_db_tree.insert(&ps.id, bincode::serialize(&ps).unwrap().to_vec()) {
            Ok(_) => {
                if let Some(fun) = self.user_created {
                    fun(&ps.id).expect("Could not execute the delegate.");
                }
                Ok(ps)
            },
            Err(_) => Err("")
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
    fn add_user(&self, user : IdentityUser) -> Result<IdentityUser, &'static str> {
        if user.email.is_empty() && user.is_pwd_empty() {
            return Err("Email and password can't be equal to nothing.")
        }
        if self.is_id_taken(&user.id) {
            return Err("User ID has already taken")
        }
        if !validator::validate_email(&user.email) {
            return Err("The email isn't valid")
        }
        if self.is_email_taken(&user.email) {
            return Err("User already exists, this can't be added");
        }
        self.user_db_tree.insert(&user.id, bincode::serialize(&user).unwrap().to_vec()).unwrap();
        Ok(user)
    }

    /**
     * Returns a bool saying if an email is already taken in the database.
     */
    fn is_email_taken(&self,email : &str) -> bool {
        self.user_db_tree
            .iter()
            .any(|ps| IdentityUser::from(ps.unwrap().1).email == email)
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
        .map(|ps|bincode::deserialize::<IdentityUser>(&ps.unwrap().1).unwrap())
        .find(|ps| ps.email == email)
    }
    
    /**
     * Returns a user based on a key or his id, if none has the key the a None is returned.
     */
    fn get_user_by_uuid(&self, uuid : &str) -> Option<IdentityUser> {
        match self.user_db_tree.get(uuid).unwrap() {
            Some(user) => Some(IdentityUser::from(user)),
            None => None
        }
    }

    /**
     * Updates an user based on his id or key in the sled database. If the update is successfull it will return a boolean and if the id of the user can't be found an error will be returned.
     */
    fn update_user(&self, id : &str, user : &IdentityUser) -> Result<bool, &'static str> {
        if let Some(mut _old_user) = self.get_user_by_uuid(id) {
            _old_user.email = user.email.to_owned();
            _old_user.last_name = user.last_name.to_owned();
            _old_user.first_name = user.first_name.to_owned();
            _old_user.hashed_password = user.hashed_password.to_owned();
            _old_user.security_stamp = user.security_stamp.to_owned();
            return Ok(
                self.user_db_tree.insert(
                    &id,
                    bincode::serialize(&_old_user).unwrap().to_vec())
                    .is_ok()
            );
        }
        Err("No user could be updated because he didn't exist")
    }
    
    /**
     * Deletes an user based on its id.
     * 
     * An error is thrown when the id is nothing or when its equal to the admin id.
     */
    fn delete_user(&self, id : &str) -> Result<bool, &'static str> {
        if id.is_empty() {
            return Err("Id can't be equal to nothing.")
        }
        if id == RESERVED_ID {
            return Err("Id can't be equal to that of the admin")
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
    fn check_user_password(&self, email : &str, pwd : &str) -> Result<bool, &'static str> {
        if pwd.is_empty() {
            return Err("Email and password can't be equal to nothing.")
        }
        if !validator::validate_email(email) {
            return Err("The email isn't valid")
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
    fn get_admin(&self) -> Result<IdentityUser,&'static str> {
        self.get_user_by_uuid(RESERVED_ID).ok_or("Admin has not been set")
    }

    /**
     * Returns a collection of all identity users that aren't admins
     */
    fn get_non_admin_users(&self) -> Vec<IdentityUser> {
        self.user_db_tree.iter()
        .map(|ps| bincode::deserialize::<IdentityUser>(&ps.unwrap().1).unwrap())
        .filter(|ps| ps.id != RESERVED_ID)
        .collect()
    }
}

#[test]
fn test_update() {
    let db = UserStore::new_db(UserConfig::new_config("","",100000),None);
    
    let mut ps = db.add_user(IdentityUser::new_user("michael@outlook.be","","","hertsens").unwrap()).unwrap();
    assert_eq!(ps.email,"michael@outlook.be");
    assert!(db.check_user_password("michael@outlook.be", "hertsens").unwrap());

    ps.set_email("michael@michael.be").unwrap();
    ps.set_password("michael@michael.be").unwrap();
    db.update_user(&ps.id, &ps.to_owned()).unwrap();
    ps = db.get_user_by_email("michael@michael.be").unwrap();

    assert_eq!(ps.email,"michael@michael.be");
    assert!(db.check_user_password("michael@michael.be", "michael@michael.be").unwrap());
}