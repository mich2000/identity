use sled::Db;

/**
 * Config containing the sled database config and the tree where it would be opened.
*/
#[derive(Clone)]
pub struct UserConfig(Db,String);

impl UserConfig {
    /**
     * Returns the UserConfig for this the path of the database and tree has to be given aswell the compression factor.
    */
    pub fn new_config(path : &str,tree : &str, cache : u64) -> UserConfig {
        if path.is_empty() {
            return UserConfig(sled::Config::new().temporary(true).cache_capacity(cache).open().unwrap(),tree.to_owned())
        }
        UserConfig(
            sled::Config::new()
            .path(path)
            .cache_capacity(cache)
            .open()
            .unwrap()
            ,tree.to_owned(),
        )
    }

    /**
     * returns a database, mostly used to generated unique id's.
     */
    pub fn get_db(&self) -> &Db { &self.0 }

    /**
     * returns the tree
    */
    pub fn get_tree(&self) -> String { self.1.clone() }
}