use sqlx::{PgPool, query_scalar};

use crate::domain::repository::user_repository::UserRepository;

pub struct PostgresqlUserRepository {
    pub pool: PgPool,
}

impl PostgresqlUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    async fn _count(&mut self) -> Result<i64, sqlx::Error> {
        let count: i64 = query_scalar!(
            r#"
                SELECT COUNT(*) from users;
            "#
        )
        .fetch_one(&self.pool)
        .await?
        .expect("somethisng went wrong");

        Ok(count)
    }
}

impl UserRepository for PostgresqlUserRepository {
    async fn create_user(
        &mut self,
        _: crate::domain::entities::user::User,
    ) -> Result<(), actix_web::Error> {
        todo!()
    }

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
    async fn postgresql_user_repository_count_test() {
        // cargo test postgresql_user_repository_count_test -- --ignored
        dotenv().ok();
        let settings = Settings::load();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&settings.postgresql_url)
            .await
            .expect("not connected");

        let mut repo = PostgresqlUserRepository::new(pool);

        let count = repo.count().await.expect("no error");

        assert_eq!(count, 0);
    }
}
