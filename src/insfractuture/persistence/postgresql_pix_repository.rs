use crate::domain::repository::pix_repository::{self, PixRepository};

struct PostgresqlPixRepository {}

impl PixRepository for PostgresqlPixRepository {
    async fn generate_url(&mut self, reais: u32) -> String {
        return "url".to_string();
    }
}
