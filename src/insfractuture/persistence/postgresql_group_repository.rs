use sqlx::{PgPool, query, query_as, query_scalar};
use uuid::Uuid;

use crate::insfractuture::persistence::mappers;

use crate::{
    domain::{entities::group::Group, repository::group_repository::GroupRepository},
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

    async fn _find_by_id(&mut self, id: Uuid) -> Result<GroupRow, sqlx::Error> {
        let group = query_as!(
            GroupRow,
            r#"
            select * from groups where id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(group)
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

    async fn find_by_id(
        &mut self,
        id: &crate::domain::value_object::group_id::GroupId,
    ) -> Result<Group, actix_web::Error> {
        let group_row = self
            ._find_by_id(id.as_uuid())
            .await
            .expect("can't find the user");

        let group: Group = group_row.into();

        Ok(group)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    use crate::{
        domain::{
            entities::group::Group,
            value_object::{group_id::GroupId, hashed_password::HashedPassword},
        },
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

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_find_by_id_test() {
        // cargo test postgresql_find_by_id_test -- --ignored --nocapture
        //a0a4e7cc-aca4-4865-ae08-70d04cea1ed4
        dotenv().ok();
        let settings = Settings::load();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&settings.postgresql_url)
            .await
            .expect("not connected");

        let mut repo = PostgresqlGroupRepository::new(pool);

        let uuid = Uuid::parse_str("a0a4e7cc-aca4-4865-ae08-70d04cea1ed4").unwrap();

        let id = GroupId::from_uuid(uuid);

        let group = repo.find_by_id(&id).await.expect("fetched");

        print!("{:#?}", group);
    }
}
