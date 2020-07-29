use identity_dal::user::identity_user::IdentityUser;
use identity_dal::user::identity_user::RESERVED_ID;
use identity_dal::traits::t_user::UserTrait;

/**
 * Viewmodel representing important attributes of the user.
 *
 * Attributes:
 * * id user
 * * email of the user
 * * first name of the user
 * * last name of the user
 * * flags of the user
 */
#[derive(serde::Serialize)]
pub struct PersonInfoViewModel {
    id: String,
    email: String,
    first_name: String,
    last_name: String,
    is_admin: bool,
}

impl PersonInfoViewModel {
    /**
     * From a IdentiyUser instance it creates a personinfo viewmodel instance.
     */
    pub fn from_identity_user(user: &IdentityUser) -> Self {
        PersonInfoViewModel {
            id: user.get_id().to_string(),
            email: user.get_email().to_string(),
            first_name: user.get_first_name().to_string(),
            last_name: user.get_last_name().to_string(),
            is_admin: user.get_id() == RESERVED_ID,
        }
    }

    pub fn get_email(&self) -> &str { &self.email }

    pub fn get_first_name(&self) -> &str { &self.first_name }

    pub fn get_last_name(&self) -> &str { &self.last_name }
    
    pub fn is_admin(&self) -> bool { self.is_admin }
}
