/**
 * Viewmodel containing a token from which a claim can be made.
 */
#[derive(serde::Deserialize)]
pub struct TokenHolderViewModel {
    token: String,
}

impl crate::traits::token::TokenContainerTrait for TokenHolderViewModel {
    fn get_token(&self) -> &str {
        &self.token
    }
}
