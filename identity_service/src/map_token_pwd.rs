use std::collections::HashMap;
use chrono::prelude::*;
use identity_dal::util::get_hash;
use chrono::Duration;
use std::sync::Mutex;

/**
 * Struct that has a hashmap that will link a token to the user id and expiration date for changing the password that an user requests.
 */
pub struct HashMapTokenPasswordChange {
    // HashMap<Token used to change the password, (user id, date and time it expires)>
    token_map : HashMap<String, (String, DateTime<Utc>)>,
    duration_change_password : i64
}

pub type TokenHolderForgottenPwd = Mutex<HashMapTokenPasswordChange>;

impl HashMapTokenPasswordChange {
    /**
     * Returns a HashMapTokenPasswordChange with a parameter that will decide how much time it will be left until it is expired.
     */
    pub fn new(duration_change : i64) -> Self {
        Self {
            token_map : HashMap::with_capacity(50),
            duration_change_password : duration_change
        }
    }

    pub fn get_duration_change_password(&self) -> i64 {
        self.duration_change_password
    }

    /**
     * gets a user id through the token, this is a key
     */
    pub fn get_user_id_from_token(&self, token : &str) -> Option<String> {
        if let Some(value) = self.token_map.get(token) {
            return Some(value.0.to_owned())
        }
        None
    }

    /**
     * Inserts a new user request to change password. This will add a token to the hashmap of the struct HashMapTokenPasswordChange as a key and the value will the be the user id and the expiration date for the password changing request.
     */
    pub fn insert_new_user_request(&mut self, user_id : &str) -> Option<String> {
        let token = get_hash(40);
        self.token_map.insert(token.to_owned(), (user_id.to_owned(),Utc::now() + Duration::seconds(self.duration_change_password)))?;
        Some(token)
    }

    /**
     * Returns a true if the token exists in the hashmap and if the expiration date of the token has gone over the now time.
     */
    pub fn is_token_okay(&mut self, token : &str) -> bool {
        if let Some(value) = self.token_map.get(token) {
            if Utc::now() > value.1 {
                self.token_map.remove(token);
                return false
            }
            return true
        }
        false
    }

    /**
     * Deletes a token if it exists and returns a true, if it doesn't exist then a false is returned.
     */
    pub fn delete_token(&mut self, token : &str) -> bool {
        if self.is_token_okay(token) {
            self.token_map.remove(token);
            return true
        }
        false
    }

    /**
     * Deletes all the entries whom user id equals the given parameter
     */
    pub fn delete_tokens_from_id(&mut self, user_id : &str) {
        self.token_map.retain(|_, value| {
            value.0 != user_id
        })
    }

    /**
     * Cleans all the tokens where the the datetime is over the one that is the now.
     */
    pub fn clean_tokens_up(&mut self) {
        let today = Utc::now();
        self.token_map.retain(|_, value| {
            today > value.1
        })
    }
}

#[test]
fn test_clean_up_id() {
    let mut map_token = HashMapTokenPasswordChange::new(45);
    map_token.insert_new_user_request("lol");
    map_token.insert_new_user_request("lol");
    map_token.insert_new_user_request("dsq");
    map_token.delete_tokens_from_id("lol");
    assert!(map_token.token_map.len() == 1);
}

#[test]
fn test_clean_up_time() {
    let mut map_token = HashMapTokenPasswordChange::new(0);
    map_token.insert_new_user_request("lol");
    map_token.insert_new_user_request("lol");
    map_token.insert_new_user_request("dsq");
    map_token.clean_tokens_up();
    assert!(map_token.token_map.len() == 0);
}