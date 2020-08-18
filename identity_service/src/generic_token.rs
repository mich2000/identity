use serde::Deserialize;
use crate::traits::token::TokenContainerTrait;

/**
 * Viewmodel GenericTokenViewModel this one takes a token and a viewmodel;
 */
#[derive(Deserialize)]
pub struct GenericTokenViewModel<T> {
    token : String,
    model : T
}

impl<T : Send + 'static> GenericTokenViewModel<T> {
    pub fn get_model(&self) -> &T {
        &self.model
    }
}

impl<T : Send + 'static> TokenContainerTrait for GenericTokenViewModel<T> {
    fn get_token(&self) -> &str {
        &self.token
    }
}