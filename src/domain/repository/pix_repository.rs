pub trait PixRepository {
    fn generate_url(&mut self, reais: u32) -> impl Future<Output = String>;
}
