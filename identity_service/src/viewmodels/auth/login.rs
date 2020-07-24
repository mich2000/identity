/**
 * Login represents the viewmodel which holds the email and password of the user.
 */
#[derive(serde::Deserialize)]
pub struct LoginViewModel {
    email: String,
    password: String,
}

impl LoginViewModel {
    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}
