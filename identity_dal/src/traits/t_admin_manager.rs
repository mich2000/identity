use crate::traits::t_user::UserTrait;
use crate::err::IdentityError;

pub trait AdminStoreTrait<T : UserTrait> {
    fn get_amount_of_non_admin_users(&self) -> usize;

    fn is_id_admin(&self,id : &str) -> bool;

    fn get_admin(&self) -> Result<T,IdentityError>;

    fn get_non_admin_users(&self) -> Vec<T>;
}