use identity_dal::repo::user_config::UserConfig;
use identity_dal::repo::user_repo::UserStore;
use identity_dal::traits::t_user_manager::UserStoreTrait;
use crate::IdentityError;
use crate::util::get_value_from_key;
use crate::service::mail_service::MailTransport;
/**
 * Struct used to provide user store to manage user's to those who want to. The struct has a config this will be used to give out the different stores.
*/
pub struct StoreManager(UserConfig);

impl Default for StoreManager {
    /**
     * default store is temporary without any compression.
    */
    fn default() -> Self {
        StoreManager(UserConfig::new_config("","person",60))
    }
}

impl StoreManager {
    /**
     * Function used to initialise the store manager this needs a tree for the database and a .env config file to make the config that will produce the user stores. If the tree is empty or the .env config file is not in a good format a panic is thrown.
     */
    pub fn new() -> StoreManager {
        StoreManager(
            UserConfig::new_config(
                &get_value_from_key("PERSON_DATABASE")
                .expect("PERSON_SMTP_PASSWORD variable not found in the .env config file or as environment variable")
                ,"person", get_value_from_key("PERSON_CACHE")
                .expect("PERSON_SMTP_PASSWORD variable not found in the .env config file or as environment variable")
                .parse::<u64>().expect("Could not parse the string to the u64 type."))
        )
    }

    /**
     * Function used to initialise the store manager this needs a tree for the database and a .env config file to make the config that will produce the user stores. If the tree is empty or the .env config file is not in a good format a panic is thrown. Before returning the store, it will do the setup.
     */
    pub fn new_with_setup() -> StoreManager {
        let store = StoreManager(
            UserConfig::new_config(
                &get_value_from_key("PERSON_DATABASE")
                .expect("PERSON_SMTP_PASSWORD variable not found in the .env config file or as environment variable")
                ,"person", get_value_from_key("PERSON_CACHE")
                .expect("PERSON_SMTP_PASSWORD variable not found in the .env config file or as environment variable")
                .parse::<u64>().expect("Could not parse the string to the u64 type."))
        );
        store.control_setup().expect("Could not execute a control setup.");
        store
    }

    /**
     * The store manager sends out a store that can be used by otherss
     */
    pub fn give_store(&self) -> Store {
        UserStore::new_db(self.0.clone())
    }

    /**
     * Uses the database and generates a string id
     */
    pub fn give_unique_id(&self) -> String {
        (self.0).get_db().generate_id().unwrap().to_string()
    }

    /**
     * Setups the admin in the sled database.
     */
    pub fn control_setup(&self) -> Result<(), IdentityError> {
        self.give_store().setup()
    }
}

/**
 * type representing the user store
 */
pub type Store = UserStore;

pub type UserDelegate = Option<fn(id : &str,store : &Store, &MailTransport) -> Result<(),IdentityError>>;