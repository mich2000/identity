use crate::traits::t_user::UserTrait;
use crate::err::IdentityError;

pub trait UserStoreTrait<T : UserTrait> {
    fn setup(&self) -> Result<(),IdentityError>;

    fn create_user_with_personal_id(&self, id : &str, email : &str, pwd : &str) -> Result<T, IdentityError>;

    fn create_user(&self, email : &str, pwd : &str) -> Result<T, IdentityError>;

    fn add_user(&self, t : T) -> Result<T, IdentityError>;
    
    fn is_email_taken(&self,email : &str) -> bool;

    fn is_id_taken(&self, id : &str) -> bool;
    
    fn get_user_by_email(&self, email : &str) -> Option<T>;
    
    fn get_user_by_uuid(&self, uuid : &str) -> Option<T>;

    fn update_user(&self, id : &str, user : &T) -> Result<bool, IdentityError>;
    
    fn delete_user(&self, id : &str) -> Result<bool, IdentityError>;
    
    fn check_user_password(&self, email : &str, pwd : &str) -> Result<bool, IdentityError>;
}