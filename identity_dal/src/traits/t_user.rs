use crate::user::identity_user::IdentityUser;

pub trait UserTrait {
    fn admin() -> Result<IdentityUser,&'static str>;

    fn new_user(email : &str,first_name : &str, last_name : &str, pwd : &str) -> Result<IdentityUser,&'static str>;

    fn new_user_with_personal_id(id : &str, email : &str,first_name : &str, last_name : &str, pwd : &str) -> Result<IdentityUser,&'static str>;
    
    fn set_password(&mut self, new_pwd : &str) -> Result<(),&'static str>;
    
    fn check_pwd(&self, pwd : &str) -> bool;
    
    fn set_user_name(&mut self, first_name : &str, last_name : &str) -> Result<(),&'static str>;
    
    fn get_id(&self) -> &str;
    
    fn get_email(&self) -> &str;
    
    fn get_hashed_password(&self) -> &str;
    
    fn get_security_stamp(&self) -> &str;
    
    fn get_first_name(&self) -> &str;
    
    fn set_email(&mut self, new_email : &str) -> Result<bool,&'static str>;

    fn get_last_name(&self) -> &str;
    
    fn set_first_name(&mut self,first_name : &str);

    fn set_last_name(&mut self,last_name : &str);

    fn set_hashed_password(&mut self,hashed_password : &str);

    fn set_security_stamp(&mut self,security_stamp : &str);
}