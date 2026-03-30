use crate::domain::value_object::pix_response::PixResponse;

#[derive(Debug, PartialEq)]
pub enum PixClientError {
    LessOrEqualZero,
    GatewayError,
    JsonParserError,
}

pub trait PixClientRepository {
    fn create_pix(
        &mut self,
        amount: f64,
    ) -> impl Future<Output = Result<PixResponse, PixClientError>>;
}
