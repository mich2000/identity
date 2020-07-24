use crate::traits::t_user::UserTrait;

pub trait AdminStoreTrait<T : UserTrait> {
    fn get_amount_of_non_admin_users(&self) -> usize;

    fn is_id_admin(&self,id : &str) -> bool;

    fn get_admin(&self) -> Result<T,&'static str>;

    fn get_non_admin_users(&self) -> Vec<T>;
}