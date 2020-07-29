use serde::{Serialize, Deserialize};
use validator::Validate;
use std::convert::From;
use crate::traits::t_user::UserTrait;
use argon2::Config;
use rand::{thread_rng, Rng};

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
#[derive(Serialize,Validate, Deserialize, Debug,Clone,PartialEq,PartialOrd)]
pub struct IdentityUser {
    id : String,
    #[validate(email)]
    email : String,
    hashed_password : String,
    security_stamp : String,
    first_name : String,
    last_name : String
}

impl From<sled::IVec> for IdentityUser {
    fn from(item : sled::IVec) -> Self {
        bincode::deserialize(&item).unwrap()
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

static HEXA_ALPHABET : [char;16] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'];

/**
 * Returns a 8 character hash made off hexadecimal characters.
 */
fn get_hash(amount : usize) -> String {
    (0..amount).map(|_| HEXA_ALPHABET[thread_rng().gen_range(0, HEXA_ALPHABET.len())] as char ).collect()
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
    fn get_first_name(&self) -> &str { &self.first_name }

    //returns a reference of the user's last name
    fn get_last_name(&self) -> &str { &self.last_name }

    fn set_first_name(&mut self,first_name : &str) {
        self.first_name = first_name.to_owned();
    }

    fn set_last_name(&mut self,last_name : &str) {
        self.last_name = last_name.to_owned();
    }

    fn set_hashed_password(&mut self,hashed_password : &str) {
        self.hashed_password = hashed_password.to_owned();
    }

    fn set_security_stamp(&mut self,security_stamp : &str) {
        self.security_stamp = security_stamp.to_owned();
    }

    /**
     * Returns the user that functions as a admin. This user with the email.admin@server.com and password ADMIN. the user has also the Admin id which is reserved and can't be given to other users.
    */
    fn admin() -> Result<IdentityUser,&'static str> {
        let hash : String = get_hash(8);
        let hashed_pwd : String = match argon2::hash_encoded(RESERVED_ID.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(hash) => hash,
            Err(_e) => return Err("Password couldn't be made") 
        };
        info!("Admin has been created. id: {}", &RESERVED_ID);
        Ok(IdentityUser {
            id : RESERVED_ID.to_owned(),
            email : "email.admin@server.com".to_string(),
            hashed_password : hashed_pwd,
            security_stamp : hash,
            first_name : "".to_string(),
            last_name : "".to_string()
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
    fn new_user_with_personal_id(id : &str, email : &str,first_name : &str, last_name : &str, pwd : &str) -> Result<IdentityUser,&'static str> {
        if email.is_empty() {
            return Err("An email can't be equal to nothing")
        }
        if pwd.is_empty() {
            return Err("An password can't be equal to nothing")
        }
        if !validator::validate_email(email) {
            return Err("The email format is not good")
        }
        let hash : String = get_hash(8);
        let hashed_pwd : String = match argon2::hash_encoded(pwd.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(hash) => hash,
            Err(_) => return Err("Password couldn't be made.") 
        };
        info!("User with personalised id has been made. id: {}",&id);
        Ok(IdentityUser {
            id : id.to_owned(),
            email : email.to_string(),
            hashed_password : hashed_pwd,
            security_stamp : hash,
            first_name : first_name.to_string(),
            last_name : last_name.to_string()
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
    fn new_user(email : &str, first_name : &str, last_name : &str, pwd : &str) 
    -> Result<IdentityUser,&'static str> {
        if email.is_empty() {
            return Err("An email can't be equal to nothing")
        }
        if pwd.is_empty() {
            return Err("An password can't be equal to nothing")
        }
        if !validator::validate_email(email) {
            return Err("The email format is not good")
        }
        let hash : String = get_hash(8);
        let hashed_pwd : String = match argon2::hash_encoded(pwd.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(hash) => hash,
            Err(_e) => return Err("Password couldn't be made") 
        };
        let user_id = get_hash(21);
        info!("A user has been added. id: {}", &user_id);
        Ok(IdentityUser {
            id : user_id,
            email : email.to_string(),
            hashed_password : hashed_pwd,
            security_stamp : hash,
            first_name : first_name.to_string(),
            last_name : last_name.to_string()
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
    fn set_password(&mut self, new_pwd : &str) -> Result<(),&'static str> {
        if new_pwd.is_empty() {
            return Err("An password can't be equal to nothing")
        }
        let hash : String = get_hash(8);
        let hashed_pwd : String = match argon2::hash_encoded(new_pwd.as_bytes(), hash.as_bytes(), &Config::default()) {
            Ok(_new_password) => _new_password,
            Err(_e) => return Err("Password couldn't be hashed")
        };
        self.security_stamp = hash;
        self.hashed_password = hashed_pwd;
        Ok(())
    }

    /**
     * Modifies the email of the users, if the user's email is different from the old a true is returned and if the same then it is false. An error is thrown when the email is empty or has a bad format. 
     */
    fn set_email(&mut self, new_email : &str) -> Result<bool,&'static str> {
        if new_email.is_empty() {
            return Err("An email can't be equal to nothing")
        }
        if self.email == new_email {
            return Ok(false)
        }
        if !validator::validate_email(new_email) {
            return Err("the new email is not in the good format or is the same")
        }
        self.email = new_email.to_owned();
        Ok(true)
    }

    /**
     * Modifies the last and first name of the user. An error is thrown when the first and last name are empty.
     */
    fn set_user_name(&mut self, first_name : &str, last_name : &str) -> Result<(),&'static str> {
        if first_name.is_empty() && last_name.is_empty() {
            return Err("The new first and last name can't be empty")
        }
        if !first_name.is_empty() {
            self.first_name = first_name.to_owned();
        }
        if !last_name.is_empty() {
            self.last_name = last_name.to_owned();
        }
        Ok(())
    }
}

#[test]
fn test_person() {
    let ps = IdentityUser::new_user("michael@michael.com", "michael", "hertsens", "Mich").unwrap();
    assert_eq!("michael@michael.com",ps.email);
    assert_eq!("michael",ps.first_name);
    assert_eq!("hertsens",ps.last_name);
}

#[test]
fn test_password() {
    let mut ps = IdentityUser::new_user("michael@michael.com", "michael", "hertsens", "Mich").unwrap();
    assert_eq!(ps.check_pwd("Mich"),true);
    assert_eq!(ps.check_pwd("ds"),false);
    ps.set_password("Mich2000").unwrap();
    assert_eq!(ps.check_pwd("Mich2000"),true);
    assert!(!ps.check_pwd("ds"));
}

#[test]
fn test_user_info() {
    let mut ps = IdentityUser::new_user("michael@michael.com", "michael", "hertsens", "Mich").unwrap();
    assert_eq!(ps.email,"michael@michael.com");
    assert_eq!(ps.first_name,"michael");
    assert_eq!(ps.last_name,"hertsens");
    ps.set_email("michael@hertsens.com").unwrap();
    assert_eq!(ps.email,"michael@hertsens.com");
    ps.set_user_name("pedro","").unwrap();
    assert_eq!(ps.first_name,"pedro");
    ps.set_user_name("", "pedro").unwrap();
    assert_eq!(ps.last_name,"pedro");
}

/**
 * Implement NaiveDate serializing and deserializing: https://serde.rs/custom-date-format.html
 */
#[allow(dead_code)]
mod basic_iso_date {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate,serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,
    {
        serializer.serialize_str(&format!("{}", date.format(FORMAT)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error> where  D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}