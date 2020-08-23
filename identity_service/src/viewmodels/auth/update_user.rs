/**
 * Viewmodel containing a token, and optionally has a email, first and last name, this is needed to update an user's information.
 */
#[derive(serde::Deserialize)]
pub struct UpdateUserViewModel {
    #[serde(default)] pub new_email: Option<String>,
    #[serde(default)] pub new_user_name: Option<String>,
}