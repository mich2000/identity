use crate::user::identity_user::IdentityUser;

pub trait UserTrait {
    fn admin() -> Result<IdentityUser,&'static str>;

    fn new_user(email : &str,first_name : &str, last_name : &str, pwd : &str) -> Result<IdentityUser,&'static str>;

    fn new_user_with_personal_id(id : &str, email : &str,first_name : &str, last_name : &str, pwd : &str) -> Result<IdentityUser,&'static str>;
    
    fn set_password(&mut self, new_pwd : &str) -> Result<(),&'static str>;
    
    fn check_pwd(&self, pwd : &str) -> bool;

    fn set_email(&mut self, new_email : &str) -> Result<bool,&'static str>;

    fn set_user_name(&mut self, first_name : &str, last_name : &str) -> Result<(),&'static str>;
}