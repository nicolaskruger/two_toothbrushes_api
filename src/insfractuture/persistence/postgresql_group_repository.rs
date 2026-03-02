use sqlx::{PgPool, query_scalar};

use crate::domain::repository::group_repository::GroupRepository;

pub struct PostgresqlGroupRepository {
    pool: PgPool,
}

impl PostgresqlGroupRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn _count(&mut self) -> Result<i64, sqlx::Error> {
        let count: i64 = query_scalar!(
            r#"
                SELECT COUNT(*) from groups;
            "#
        )
        .fetch_one(&self.pool)
        .await?
        .expect("somethisng went wrong");

        Ok(count)
    }
}

impl GroupRepository for PostgresqlGroupRepository {
    async fn count(&mut self) -> Result<i64, actix_web::Error> {
        let count = self._count().await.expect("somethisng went wrong ");

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;

    use crate::insfractuture::config::settings::Settings;

    use super::*;

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_group_repository_count_test() {
        // cargo test postgresql_group_repository_count_test -- --ignored
        dotenv().ok();
        let settings = Settings::load();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&settings.postgresql_url)
            .await
            .expect("not connected");

        let mut repo = PostgresqlGroupRepository::new(pool);

        let count = repo.count().await.expect("no error");

        assert_eq!(count, 0);
    }
}
