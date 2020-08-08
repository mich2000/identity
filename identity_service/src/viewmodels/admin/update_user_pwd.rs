/**
 * Admin viewmodel used to change the password of an user. The token is used to make sure it is authorized.
 * **/
#[derive(serde::Deserialize)]
pub struct AdminChangePasswordUserViewModel {
    id_user: String,
    password: String,
    confirm_password: String
}

impl AdminChangePasswordUserViewModel {
    pub fn get_id_user(&self) -> &str {
        &self.id_user
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_confirm_password(&self) -> &str {
        &self.confirm_password
    }
}