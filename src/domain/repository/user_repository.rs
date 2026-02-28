use actix_web::Error;

use crate::domain::entities::user::User;

pub trait UserRepository {
    fn create_user(&mut self, user: User) -> impl Future<Output = Result<(), Error>>;
}
