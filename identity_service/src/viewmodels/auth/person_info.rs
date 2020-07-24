use identity_dal::user::identity_user::IdentityUser;
use identity_dal::user::identity_user::RESERVED_ID;

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
            id: user.id.clone(),
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            is_admin: user.id == RESERVED_ID,
        }
    }
}
