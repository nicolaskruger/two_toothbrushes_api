use actix_web::Error;

pub trait GroupRepository {
    fn count(&mut self) -> impl Future<Output = Result<i64, Error>>;
}
