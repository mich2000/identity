/**
 * Admin viewmodel used to change the password of an user.
 */
#[derive(serde::Deserialize)]
pub struct AdminCreateUserViewModel {
    token_admin: String,
    email_user: String,
    password: String,
    confirm_password: String,
}

impl AdminCreateUserViewModel {
    pub fn get_email(&self) -> &str {
        &self.email_user
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_confirmed_password(&self) -> &str {
        &self.confirm_password
    }
}

impl crate::traits::token::TokenContainerTrait for AdminCreateUserViewModel {
    fn get_token(&self) -> &str {
        &self.token_admin
    }
}
