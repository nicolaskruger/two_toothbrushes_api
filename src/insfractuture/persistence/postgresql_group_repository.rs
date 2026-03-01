use crate::domain::{entities::payment::Payment, repository::pix_repository::PixRepository};

pub struct PostgresqlPixRepository {}

impl PixRepository for PostgresqlPixRepository {
    async fn register_payment(&mut self, _: Payment) -> Result<(), actix_web::Error> {
        todo!()
    }
}
