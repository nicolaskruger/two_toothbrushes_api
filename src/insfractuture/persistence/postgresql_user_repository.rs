use sqlx::{PgPool, query_scalar};

use crate::domain::repository::group_repository::GroupRepository;

pub struct PostgresqlGroupRepository {
    pool: PgPool,
}

impl PostgresqlGroupRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn _count(&mut self) -> Result<i64, sqlx::Error> {
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
