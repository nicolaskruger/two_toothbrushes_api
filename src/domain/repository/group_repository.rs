use actix_web::Error;

use crate::domain::entities::group::Group;

pub trait GroupRepository {
    fn count(&mut self) -> impl Future<Output = Result<i64, Error>>;
    fn create(&mut self, group: &Group) -> impl Future<Output = Result<(), Error>>;
}
