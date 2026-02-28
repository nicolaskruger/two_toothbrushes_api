use actix_web::Error;

use crate::domain::entities::payment::Payment;

pub trait PixRepository {
    fn register_payment(&mut self, payment: Payment) -> impl Future<Output = Result<(), Error>>;
}
