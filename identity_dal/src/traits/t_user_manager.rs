use crate::traits::t_user::UserTrait;

pub trait UserStoreTrait<T : UserTrait> {
    fn setup(&self) -> Result<(),&'static str>;

    fn create_user_with_personal_id(&self, id : &str, email : &str, pwd : &str) -> Result<T, &'static str>;

    fn create_user(&self, email : &str, pwd : &str) -> Result<T, &'static str>;

    fn add_user(&self, t : T) -> Result<T, &'static str>;
    
    fn is_email_taken(&self,email : &str) -> bool;

    fn is_id_taken(&self, id : &str) -> bool;
    
    fn get_user_by_email(&self, email : &str) -> Option<T>;
    
    fn get_user_by_uuid(&self, uuid : &str) -> Option<T>;

    fn update_user(&self, id : &str, user : &T) -> Result<bool, &'static str>;
    
    fn delete_user(&self, id : &str) -> Result<bool, &'static str>;
    
    fn check_user_password(&self, email : &str, pwd : &str) -> Result<bool, &'static str>;
}