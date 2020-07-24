/**
 * Viewmodel containing a token, and optionally has a email, first and last name, this is needed to update an user's information.
 */
#[derive(serde::Deserialize)]
pub struct UpdateUserViewModel {
    token: String,
    #[serde(default)]
    pub new_email: Option<String>,
    #[serde(default)]
    pub new_first_name: Option<String>,
    #[serde(default)]
    pub new_last_name: Option<String>,
}

impl crate::traits::token::TokenContainerTrait for UpdateUserViewModel {
    fn get_token(&self) -> &str {
        &self.token
    }
}
