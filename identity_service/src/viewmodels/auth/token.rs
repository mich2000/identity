/**
 * Viewmodel containing a token from which a claim can be made, or is also used to have a token for changing password.
 */
#[derive(serde::Deserialize,serde::Serialize)]
pub struct TokenHolderViewModel {
    token: String,
}

impl TokenHolderViewModel {
    pub fn new(tkn : &str) -> Self {
        TokenHolderViewModel {
            token : tkn.to_owned()
        }
    }
}

impl crate::traits::token::TokenContainerTrait for TokenHolderViewModel {
    fn get_token(&self) -> &str {
        &self.token
    }
}
