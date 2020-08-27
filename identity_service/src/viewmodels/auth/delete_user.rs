/**
 * Viewmodel used to delete a user based on his token which provides his user id, his password to re-confirm his choice and boolean to assess the choice.
*/
#[derive(serde::Deserialize)]
pub struct DeleteUserViewModel {
    password: String,
    delete_confirmed: bool,
}

impl DeleteUserViewModel {
    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn is_delete_confirmed(&self) -> bool {
        self.delete_confirmed
    }
}