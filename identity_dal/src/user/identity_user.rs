use serde::{Serialize, Deserialize};
use std::convert::From;
use crate::traits::t_user::UserTrait;
use argon2::Config;
use crate::err::IdentityError;
use crate::util::get_hash;
use std::collections::BTreeSet;

//Reserved id that is used only for the admin.
pub static RESERVED_ID : &str = "ADMIN";

/**
 * IdentityUser is used to represent the user.
 * 
 * Attributes:
 * * uuid: unique identification number
 * * email
 * * hashed_password: is a hash produced off a salt and the user's password
 * * security_stamp: salt used for the hashing of the password
 * * first_name
 * * last_name
 * * flags: these are the attributes that a user can have can be both claims and roles.
 */
#[derive(Serialize, Deserialize, Debug,Clone,PartialEq,PartialOrd,Eq,Hash)]
pub struct IdentityUser {
    id : String,
    email : String,
    hashed_password : String,
    security_stamp : String,
    user_name : String,
    flags : BTreeSet<String>
}

impl From<&sled::IVec> for IdentityUser {
    fn from(item : &sled::IVec) -> Self {
        bincode::deserialize(&item).expect("Could not deserialize the bytes to a struct.")
    }
}

impl From<&IdentityUser> for sled::IVec {
    fn from(item : &IdentityUser) -> Self {
        sled::IVec::from(bincode::serialize(item).expect("Could not serialize the struct to a byte vector"))
    }
}

impl IdentityUser {
    /**
     * Returns a bool if the hashed password and salt of the user's is empty
     */
    pub fn is_pwd_empty(&self) -> bool {
        self.hashed_password.is_empty() && self.security_stamp.is_empty()
    }
}

impl UserTrait for IdentityUser {
    //returns a reference of the user's id
    fn get_id(&self) -> &str { &self.id }

    //returns a reference of the user's email
    fn get_email(&self) -> &str { &self.email }

    //returns a reference of the user's hashed password
    fn get_hashed_password(&self) -> &str { &self.hashed_password }

    //returns a reference of the user's security stamp also known as the salt
    fn get_security_stamp(&self) -> &str { &self.security_stamp }

    //returns a reference of the user's first name
    fn get_user_name(&self) -> &str { &self.user_name }

    fn set_user_name(&mut self,new_user_name : &str) {
        self.user_name = new_user_name.to_owned();
    }

    fn set_hashed_password(&mut self,hashed_password : &str) {
        self.hashed_password = hashed_password.to_owned();
    }

    fn set_security_stamp(&mut self,security_stamp : &str) {
        self.security_stamp = security_stamp.to_owned();
    }

    fn set_flags(&mut self, flags : BTreeSet<String>) {
        self.flags = flags;
    }

    /**
     * Returns the user that functions as a admin. This user with the email.admin@server.com and password ADMIN. the user has also the Admin id which is reserved and can't be given to other users.
    */
    fn admin() -> Result<IdentityUser,IdentityError> {
        let hash : String = get_hash(8);
        let hashed_pwd : String = match argon2::hash_encoded(RESERVED_ID.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(hash) => hash,
            Err(_e) => return Err(IdentityError::PasswordCannotBeMade)
        };
        info!("Admin has been created. id: {}", &RESERVED_ID);
        Ok(IdentityUser {
            id : RESERVED_ID.to_owned(),
            email : "email.admin@server.com".to_string(),
            hashed_password : hashed_pwd,
            security_stamp : hash,
            user_name : "".to_owned(),
            flags : BTreeSet::default()
        })
    }

    /**
     * Returns a new user with hashed password, email, first and last name and lets the id be chosen.
     * 
     * Returns an error when:
     * email is empty
     * password is empty
     * email has a bad format
     * **/
    fn new_user_with_personal_id(id : &str, email : &str,user_name : &str, pwd : &str) 
    -> Result<IdentityUser,IdentityError> {
        if email.is_empty() {
            return Err(IdentityError::EmailIsEmpty)
        }
        if pwd.is_empty() {
            return Err(IdentityError::PasswordIsEmpty)
        }
        if !crate::util::control_email(email) {
            return Err(IdentityError::EmailNotCorrectFormat)
        }
        let hash : String = get_hash(8);
        let hashed_pwd : String = match argon2::hash_encoded(pwd.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(hash) => hash,
            Err(_) => return Err(IdentityError::PasswordCannotBeMade) 
        };
        info!("User with personalised id has been made. id: {}",&id);
        Ok(IdentityUser {
            id : id.to_owned(),
            email : email.to_string(),
            hashed_password : hashed_pwd,
            security_stamp : hash,
            user_name : user_name.to_owned(),
            flags : BTreeSet::default()
        })
    }
    
    /**
     * Returns a new user with hashed password, email, first and last name.
     * 
     * Returns an error when:
     * email is empty
     * password is empty
     * email has a bad format
     * **/
    fn new_user(email : &str, user_name : &str, pwd : &str) 
    -> Result<IdentityUser,IdentityError> {
        if email.is_empty() {
            return Err(IdentityError::EmailIsEmpty)
        }
        if pwd.is_empty() {
            return Err(IdentityError::PasswordIsEmpty)
        }
        if !crate::util::control_email(email) {
            return Err(IdentityError::EmailNotCorrectFormat)
        }
        let hash : String = get_hash(8);
        let hashed_pwd : String = match argon2::hash_encoded(pwd.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(hash) => hash,
            Err(_e) => return Err(IdentityError::PasswordCannotBeMade) 
        };
        let user_id = get_hash(21);
        info!("A user has been added. id: {}", &user_id);
        Ok(IdentityUser {
            id : user_id,
            email : email.to_string(),
            hashed_password : hashed_pwd,
            security_stamp : hash,
            user_name : user_name.to_owned(),
            flags : BTreeSet::default()
        })
    }

    /**
     * Checks if given password is equal to the person's password
     **/
    fn check_pwd(&self, pwd : &str) -> bool {
        if pwd.is_empty() {
            return false
        }
        argon2::verify_encoded(&self.hashed_password, pwd.as_bytes()).unwrap()
    }

    /**
     * Changes the password of an user. It will return a error if the new password is empty.
     **/
    fn set_password(&mut self, new_pwd : &str) -> Result<(),IdentityError> {
        if new_pwd.is_empty() {
            return Err(IdentityError::PasswordIsEmpty)
        }
        let hash : String = get_hash(8);
        let hashed_pwd : String = match argon2::hash_encoded(new_pwd.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(_new_password) => _new_password,
            Err(_e) => return Err(IdentityError::PasswordCannotBeMade)
        };
        self.security_stamp = hash;
        self.hashed_password = hashed_pwd;
        Ok(())
    }

    /**
     * Modifies the email of the users, if the user's email is different from the old a true is returned and if the same then it is false. An error is thrown when the email is empty or has a bad format. 
     */
    fn set_email(&mut self, new_email : &str) -> Result<bool,IdentityError> {
        if new_email.is_empty() {
            return Err(IdentityError::EmailIsEmpty)
        }
        if self.email == new_email {
            return Ok(false)
        }
        if !crate::util::control_email(new_email) {
            return Err(IdentityError::EmailNotCorrectFormat)
        }
        self.email = new_email.to_owned();
        Ok(true)
    }

    fn get_flags(&self) -> BTreeSet<String> {
        self.flags.clone()
    }

    fn get_flag_list(&self) -> Vec<String> {
        self.flags.iter().map(String::from).collect()
    }

    fn add_flag(&mut self, flag : &str) -> bool {
        self.flags.insert(flag.to_owned())
    }

    fn remove_flag(&mut self, flag : &str) -> bool {
        self.flags.remove(flag)
    }
}

/**
 * Implement NaiveDate serializing and deserializing: https://serde.rs/custom-date-format.html
 */
#[allow(dead_code)]
mod basic_iso_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate,serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        serializer.serialize_str(&format!("{}", date.format(FORMAT)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error> where  D: Deserializer<'de>,
    {
        NaiveDate::parse_from_str(&String::deserialize(deserializer)?, FORMAT).map_err(serde::de::Error::custom)
    }
}