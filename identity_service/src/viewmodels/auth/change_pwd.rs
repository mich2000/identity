#[derive(serde::Deserialize)]
pub struct ChangeForgottenPassword {
    token_forgotten_pwd : String,
    password : String, 
    confirm_password : String
}

impl ChangeForgottenPassword {
    /**
     * Gets a reference to the token_forgotten_pwd property
     */
    pub fn get_token_forgotten_pwd(&self) -> &str { &self.token_forgotten_pwd }

    /**
     * Gets a reference to the password property
     */
    pub fn get_password(&self) -> &str { &self.password }

    /**
     * Gets a reference to the confirm_password property
     */
    pub fn get_confirm_password(&self) -> &str { &self.confirm_password }
}