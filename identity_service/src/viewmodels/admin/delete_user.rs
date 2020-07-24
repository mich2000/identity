/**
 * Viewmodel used to delete a user based on his token which provides his user id, his password to re-confirm his choice and boolean to assess the choice.
*/
#[derive(serde::Deserialize)]
pub struct DeleteUserViewModel {
    token: String,
    user_id: String,
}

impl DeleteUserViewModel {
    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
}

impl crate::traits::token::TokenContainerTrait for DeleteUserViewModel {
    fn get_token(&self) -> &str {
        &self.token
    }
}
