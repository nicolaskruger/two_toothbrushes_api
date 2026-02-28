use crate::domain::repository::user_repository::UserRepository;

pub struct PostgresqlUserRepository {}

impl UserRepository for PostgresqlUserRepository {
    async fn create_user(
        &mut self,
        user: crate::domain::entities::user::User,
    ) -> Result<(), actix_web::Error> {
        todo!()
    }
}
