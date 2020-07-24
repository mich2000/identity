/**
 * Viewmodel used to change the password of an user. The token is used to make sure it is authorized.
 * **/
#[derive(serde::Deserialize)]
pub struct ChangePasswordViewModel {
    token: String,
    password: String,
    confirm_password: String,
}

impl crate::traits::token::TokenContainerTrait for ChangePasswordViewModel {
    fn get_token(&self) -> &str {
        &self.token
    }
}

impl ChangePasswordViewModel {
    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_confirm_password(&self) -> &str {
        &self.confirm_password
    }
}
