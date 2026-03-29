use crate::domain::{entities::payment::Payment, value_object::pix::Pix};

#[derive(Debug, PartialEq)]
pub enum PixError {
    LessOrEqualZero,
    GatewayError,
    JsonParserError,
}

pub trait PixRepository {
    fn register_payment(&mut self, payment: Payment) -> impl Future<Output = Result<(), PixError>>;
    fn create_pix(&mut self, amount: f64) -> impl Future<Output = Result<Pix, PixError>>;
}
