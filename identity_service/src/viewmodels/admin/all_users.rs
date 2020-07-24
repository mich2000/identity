use crate::viewmodels::auth::person_info::PersonInfoViewModel;
use identity_dal::user::identity_user::IdentityUser;

#[derive(serde::Serialize)]
pub struct AllNonAdminUsersViewModel {
    pub users : Vec<PersonInfoViewModel>
}

impl AllNonAdminUsersViewModel {
    pub fn from_users_vector(users : Vec<IdentityUser>) -> Self {
        AllNonAdminUsersViewModel {
            users : users.iter().map(|ps| PersonInfoViewModel::from_identity_user(&ps)).collect()
        }
    }
}