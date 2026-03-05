use crate::domain::entities::group::Group;
use crate::domain::entities::user::User;
use crate::insfractuture::persistence::models::group_row::GroupRow;
use crate::insfractuture::persistence::models::user_row::UserRow;

impl From<&Group> for GroupRow {
    fn from(group: &Group) -> Self {
        Self {
            id: group.id().as_uuid(),
            name: group.name().to_string(),
            password: group.password().as_str().to_string(),
            created_at: group.created_at().clone(),
        }
    }
}

impl From<&User> for UserRow {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().as_uuid(),
            name: user.name().to_string(),
            is_confirm: user.is_confirm().clone(),
            created_at: user.created_at().clone(),
            group_id: user.group_id().as_uuid(),
        }
    }
}
