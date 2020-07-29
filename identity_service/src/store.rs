use identity_dal::repo::user_config::UserConfig;
use identity_dal::repo::user_repo::UserStore;
use identity_dal::traits::t_user_manager::UserStoreTrait;

/**
 * Struct used to provide user store to manage user's to those who want to. The struct has a config this will be used to give out the different stores.
*/
pub struct StoreManager(UserConfig, UserDelegate);

impl std::default::Default for StoreManager {
    /**
     * default store is temporary without any compression.
    */
    fn default() -> Self {
        StoreManager(UserConfig::new_config("","person",dotenv::var("person_cache")
        .expect("The line person_cache isn't set in the .env config file.")
        .parse::<u64>().expect("Could not parse the string to the u64 type.")),None)
    }
}

impl StoreManager {
    /**
     * Function used to initialise the store manager this needs a tree for the database and a .env config file to make the config that will produce the user stores. If the tree is empty or the .env config file is not in a good format a panic is thrown.
     */
    pub fn new(user_created : UserDelegate) -> StoreManager {
        StoreManager(
            UserConfig::new_config(
                &dotenv::var("person_database").expect("The path to the database file isn't set.")
                ,"person", dotenv::var("person_cache").expect("The line person_cache isn't set in the .env config file.")
                .parse::<u64>().expect("Could not parse the string to the u64 type.")),
                user_created
        )
    }

    pub fn give_user_creation_fun(&self) -> UserDelegate {
        self.1
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

    pub fn control_setup(&self) -> Result<(), &'static str> {
        self.give_store().setup()
    }
}

//type representing the user store
pub type Store = UserStore;

pub type UserDelegate = Option<fn(id : &str, &Store) -> Result<(),&'static str>>;