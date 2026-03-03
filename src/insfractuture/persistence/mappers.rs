use crate::domain::entities::group::Group;
use crate::insfractuture::persistence::models::group_row::GroupRow;

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
