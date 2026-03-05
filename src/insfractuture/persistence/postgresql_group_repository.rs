use sqlx::{PgPool, query, query_scalar};

use crate::{
    domain::repository::group_repository::GroupRepository,
    insfractuture::persistence::models::group_row::GroupRow,
};

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

    async fn _create(&mut self, group: &GroupRow) -> Result<(), sqlx::Error> {
        query!(
            r#"
                INSERT INTO groups (id, name, password)
                VALUES ($1, $2, $3);
            "#,
            group.id,
            group.name,
            group.password
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

impl GroupRepository for PostgresqlGroupRepository {
    async fn count(&mut self) -> Result<i64, actix_web::Error> {
        let count = self._count().await.expect("somethisng went wrong ");

        Ok(count)
    }

    async fn create(
        &mut self,
        group: &crate::domain::entities::group::Group,
    ) -> Result<(), actix_web::Error> {
        let grouo_row: GroupRow = group.into();
        self._create(&grouo_row)
            .await
            .expect("somethisng went wrong ");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;

    use crate::{
        domain::{entities::group::Group, value_object::hashed_password::HashedPassword},
        insfractuture::config::settings::Settings,
    };

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

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_group_repository_create_test() {
        // cargo test postgresql_group_repository_create_test -- --ignored
        dotenv().ok();
        let settings = Settings::load();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&settings.postgresql_url)
            .await
            .expect("not connected");

        let group = Group::create(
            "name".to_string(),
            HashedPassword::new("password".to_string()),
            Utc::now(),
        );

        let mut repo = PostgresqlGroupRepository::new(pool);

        repo.create(&group).await.expect("no error");
    }
}
