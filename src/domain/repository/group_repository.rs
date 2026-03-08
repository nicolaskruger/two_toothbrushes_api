use actix_web::Error;

use crate::domain::{entities::group::Group, value_object::group_id::GroupId};

pub trait GroupRepository {
    fn count(&mut self) -> impl Future<Output = Result<i64, Error>>;
    fn create(&mut self, group: &Group) -> impl Future<Output = Result<(), Error>>;
    fn find_by_id(&mut self, id: &GroupId) -> impl Future<Output = Result<Group, Error>>;
}
