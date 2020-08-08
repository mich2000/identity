/**
 * Viewmodel used to change the password of an user. The token is used to make sure it is authorized.
 * **/
#[derive(serde::Deserialize)]
pub struct ChangePasswordViewModel {
    password: String,
    confirm_password: String,
}

impl ChangePasswordViewModel {
    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_confirm_password(&self) -> &str {
        &self.confirm_password
    }
}
