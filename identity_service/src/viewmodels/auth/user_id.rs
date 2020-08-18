#[derive(serde::Deserialize,serde::Serialize)]
pub struct UserIdViewModel {
    user_id : String
}

impl UserIdViewModel {
    pub fn get_id(&self) -> &str { &self.user_id }
}