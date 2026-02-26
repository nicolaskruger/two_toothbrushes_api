use actix_web::Error;

use crate::domain::entities::payment::Payment;

pub trait PixRepository {
    async fn register_payment(&mut self, payment: Payment) -> Result<(), Error>;
}
