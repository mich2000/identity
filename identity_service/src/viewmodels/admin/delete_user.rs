/**
 * Viewmodel used to delete a user based on his token which provides his user id, his password to re-confirm his choice and boolean to assess the choice.
*/
#[derive(serde::Deserialize)]
pub struct DeleteUserViewModel {
    user_id: String
}

impl DeleteUserViewModel {
    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
}