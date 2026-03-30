use crate::domain::value_object::pix::Pix;

#[derive(Debug, PartialEq)]
pub enum PixError {
    OnCreateError,
}

pub trait PixRepository {
    fn create_pix(&mut self, pix: Pix) -> impl Future<Output = Result<Pix, PixError>>;
}
