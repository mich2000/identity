use identity_dal::repo::user_config::UserConfig;
use identity_dal::repo::user_repo::UserStore;
use identity_dal::traits::t_user_manager::UserStoreTrait;

/**
 * Struct used to provide user store to manage user's to those who want to. The struct has a config this will be used to give out the different stores.
*/
pub struct StoreManager(pub UserConfig);

impl std::default::Default for StoreManager {
    /**
     * default store is temporary without any compression.
    */
    fn default() -> Self {
        StoreManager(UserConfig::new_config("","person"))
    }
}

impl StoreManager {
    /**
     * Function used to initialise the store manager this needs a tree for the database and a .env config file to make the config that will produce the user stores. If the tree is empty or the .env config file is not in a good format a panic is thrown.
     */
    pub fn new() -> StoreManager {
        StoreManager(
            UserConfig::new_config(&dotenv::var("person_database").expect("The path to the database file isn't set."),"person")
        )
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