#[derive(serde::Deserialize,serde::Serialize)]
pub struct FlagHolder {
    flag : String
}

impl FlagHolder {
    pub fn get_flag(&self) -> &str { &self.flag }
}