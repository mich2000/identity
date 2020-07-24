/**
 * Viewmodel containing attributes email and password and their confirmation for registrating a new user.
 *
 * The password and their confirmation needs to be equal.
 */
#[derive(serde::Deserialize)]
pub struct RegistrationViewModel {
    email: String,
    password: String,
    confirm_password: String,
}

impl RegistrationViewModel {
    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_confirmed_password(&self) -> &str {
        &self.confirm_password
    }
}
