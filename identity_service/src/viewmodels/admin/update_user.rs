/**
 * Viewmodel containing a token and the user id that is going to get updated, and optionally has a email, first and last name, this is needed to update an user's information.
 */
#[derive(serde::Deserialize)]
pub struct AdminUpdateUserViewModel {
    token: String,
    user_id: String,
    #[serde(default)]
    pub new_email: Option<String>,
    #[serde(default)]
    pub new_first_name: Option<String>,
    #[serde(default)]
    pub new_last_name: Option<String>,
}

impl AdminUpdateUserViewModel {
    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
}

impl crate::traits::token::TokenContainerTrait for AdminUpdateUserViewModel {
    fn get_token(&self) -> &str {
        &self.token
    }
}
