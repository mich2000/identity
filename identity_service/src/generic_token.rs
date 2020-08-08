/**
 * Viewmodel GenericTokenViewModel this one takes a token and a viewmodel in that is an
 */
#[derive(serde::Deserialize)]
pub struct GenericTokenViewModel<T> {
    token : String,
    model : T
}

impl<T : Send + 'static> GenericTokenViewModel<T> {
    pub fn get_model(&self) -> &T {
        &self.model
    }
}

impl<T : Send + 'static> crate::traits::token::TokenContainerTrait for GenericTokenViewModel<T> {
    fn get_token(&self) -> &str {
        &self.token
    }
}