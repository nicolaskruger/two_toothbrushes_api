use sqlx::{PgPool, query, query_as, query_scalar};
use uuid::Uuid;

use crate::domain::repository::group_repository::GroupRepositoryError;

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
                INSERT INTO groups (id, name, code, password)
                VALUES ($1, $2, $3, $4);
            "#,
            group.id,
            group.name,
            group.code,
            group.password
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn _update(&mut self, group: &GroupRow) -> Result<(), sqlx::Error> {
        let result = query!(
            r#"
                UPDATE groups
                SET name = $2,
                    code = $3,
                    password = $4
                WHERE id = $1;
            "#,
            group.id,
            group.name,
            group.code,
            group.password
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    async fn _delete(&mut self, id: Uuid) -> Result<(), sqlx::Error> {
        let result = query!(
            r#"
                DELETE FROM groups WHERE id = $1;
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    async fn _find_all(&mut self) -> Result<Vec<GroupRow>, sqlx::Error> {
        let groups = query_as!(
            GroupRow,
            r#"
            select * from groups
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(groups)
    }

    async fn _find_by_name(&mut self, name: String) -> Result<GroupRow, sqlx::Error> {
        let group = query_as!(
            GroupRow,
            r#"
            select * from groups where name = $1
            "#,
            name,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(group)
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
    async fn count(&mut self) -> Result<i64, GroupRepositoryError> {
        let count = self
            ._count()
            .await
            .map_err(|_| GroupRepositoryError::SQLError)?;

        Ok(count)
    }

    async fn create(
        &mut self,
        group: &crate::domain::entities::group::Group,
    ) -> Result<(), GroupRepositoryError> {
        let grouo_row: GroupRow = group.into();
        self._create(&grouo_row).await.map_err(|e| match e {
            // groups.code is UNIQUE
            sqlx::Error::Database(e) if e.is_unique_violation() => {
                GroupRepositoryError::CouldNotCreate
            }
            _ => GroupRepositoryError::SQLError,
        })?;
        Ok(())
    }

    async fn find_by_id(
        &mut self,
        id: &crate::domain::value_object::group_id::GroupId,
    ) -> Result<Group, GroupRepositoryError> {
        let group_row = self
            ._find_by_id(id.as_uuid())
            .await
            .map_err(|_| GroupRepositoryError::NotFound)?;

        let group: Group = group_row.into();

        Ok(group)
    }

    async fn find_by_name(&mut self, name: String) -> Result<Group, GroupRepositoryError> {
        let group_row = self
            ._find_by_name(name)
            .await
            .map_err(|_| GroupRepositoryError::NotFound)?;

        let group: Group = group_row.into();

        Ok(group)
    }

    async fn update(
        &mut self,
        group: &crate::domain::entities::group::Group,
    ) -> Result<(), GroupRepositoryError> {
        let group_row: GroupRow = group.into();

        self._update(&group_row).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => GroupRepositoryError::NotFound,
            // groups.code is UNIQUE
            sqlx::Error::Database(e) if e.is_unique_violation() => {
                GroupRepositoryError::CouldNotCreate
            }
            _ => GroupRepositoryError::CouldNotUpdate,
        })?;

        Ok(())
    }

    async fn delete(
        &mut self,
        id: &crate::domain::value_object::group_id::GroupId,
    ) -> Result<(), GroupRepositoryError> {
        self._delete(id.as_uuid()).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => GroupRepositoryError::NotFound,
            _ => GroupRepositoryError::CouldNotDelete,
        })?;

        Ok(())
    }

    async fn find_all(&mut self) -> Result<Vec<Group>, GroupRepositoryError> {
        let group_rows = self
            ._find_all()
            .await
            .map_err(|_| GroupRepositoryError::SQLError)?;

        let groups: Vec<_> = group_rows.into_iter().map(|r| r.into()).collect();

        Ok(groups)
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
            "code".to_string(),
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

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_find_by_name_test() {
        // cargo test postgresql_find_by_name_test -- --ignored --nocapture
        //a0a4e7cc-aca4-4865-ae08-70d04cea1ed4
        dotenv().ok();
        let settings = Settings::load();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&settings.postgresql_url)
            .await
            .expect("not connected");

        let mut repo = PostgresqlGroupRepository::new(pool);

        let group = repo
            .find_by_name("Pai e Mãe".to_string())
            .await
            .expect("fetched");

        print!("{:#?}", group);
    }
}
