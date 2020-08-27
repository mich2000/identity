pub mod claim;
pub mod service;
pub mod store;
pub mod viewmodels;
pub mod mail_struct;
pub mod util;
pub mod map_token_pwd;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

extern crate lettre;
extern crate lettre_email;

pub type IdentityError = identity_dal::err::IdentityError;