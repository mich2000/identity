use crate::user::identity_user::IdentityUser;
use crate::err::IdentityError;
use std::collections::BTreeSet;

pub trait UserTrait {
    fn admin() -> Result<IdentityUser,IdentityError>;

    fn new_user(email : &str,user_name : &str, pwd : &str) -> Result<IdentityUser,IdentityError>;

    fn new_user_with_personal_id(id : &str, email : &str, user_name : &str, pwd : &str)
     -> Result<IdentityUser,IdentityError>;
    
    fn set_password(&mut self, new_pwd : &str) -> Result<(),IdentityError>;
    
    fn check_pwd(&self, pwd : &str) -> bool;
    
    fn set_user_name(&mut self, new_user_name : &str);
    
    fn get_id(&self) -> &str;
    
    fn get_email(&self) -> &str;
    
    fn get_hashed_password(&self) -> &str;
    
    fn get_security_stamp(&self) -> &str;
    
    fn set_email(&mut self, new_email : &str) -> Result<bool,IdentityError>;

    fn get_user_name(&self) -> &str;

    fn set_hashed_password(&mut self,hashed_password : &str);

    fn set_security_stamp(&mut self,security_stamp : &str);

    fn get_flags(&self) -> BTreeSet<String>;

    fn get_flag_list(&self) -> Vec<String>;

    fn set_flags(&mut self, flags : BTreeSet<String>);

    fn add_flag(&mut self, flag : &str) -> bool;

    fn remove_flag(&mut self, flag : &str) -> bool;
}