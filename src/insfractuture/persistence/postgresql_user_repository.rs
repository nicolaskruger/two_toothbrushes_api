use sqlx::{PgPool, query, query_as, query_scalar};
use uuid::Uuid;

use crate::{
    domain::{
        entities::user::User,
        repository::user_repository::{UserRepository, UserRepositoryError},
    },
    insfractuture::persistence::models::user_row::UserRow,
};

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

    async fn _create(&mut self, user_row: &UserRow) -> Result<(), sqlx::Error> {
        query!(
            r#"
                INSERT INTO users (id, name, is_confirm, group_id)
                VALUES ($1, $2, $3, $4);
            "#,
            user_row.id,
            user_row.name,
            user_row.is_confirm,
            user_row.group_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn _find_by_group(&mut self, id: Uuid) -> Result<Vec<UserRow>, sqlx::Error> {
        let users = query_as!(
            UserRow,
            r#"
            select * from users where group_id = $1
            "#,
            id,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
}

impl UserRepository for PostgresqlUserRepository {
    async fn create_user(
        &mut self,
        user: &crate::domain::entities::user::User,
    ) -> Result<(), UserRepositoryError> {
        let user_row: UserRow = user.into();

        self._create(&user_row)
            .await
            .map_err(|_| UserRepositoryError::CouldNotCreate)?;

        Ok(())
    }

    async fn count(&mut self) -> Result<i64, UserRepositoryError> {
        let count = self
            ._count()
            .await
            .map_err(|_| UserRepositoryError::SQLError)?;

        Ok(count)
    }

    async fn find_by_group(
        &mut self,
        group_id: &crate::domain::value_object::group_id::GroupId,
    ) -> Result<Vec<User>, UserRepositoryError> {
        let user_rows = self
            ._find_by_group(group_id.as_uuid())
            .await
            .map_err(|_| UserRepositoryError::SQLError)?;

        let users: Vec<_> = user_rows.iter().map(|r| r.into()).collect();

        Ok(users)
    }
}

#[cfg(test)]
mod tests {

    use chrono::Utc;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;

    use crate::{
        domain::{entities::user::User, value_object::group_id::GroupId},
        insfractuture::config::settings::Settings,
    };

    use super::*;

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_user_repository_create_test() {
        // cargo test postgresql_user_repository_create_test -- --ignored
        dotenv().ok();
        let settings = Settings::load();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&settings.postgresql_url)
            .await
            .expect("not connected");

        let mut repo = PostgresqlUserRepository::new(pool);

        let uuid = Uuid::parse_str("a0a4e7cc-aca4-4865-ae08-70d04cea1ed4").unwrap();
        let group_id = GroupId::from_uuid(uuid);

        let user = User::create("nicolas".to_string(), true, group_id, Utc::now());

        repo.create_user(&user).await.expect("error on insert");
    }

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

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_user_repository_find_by_group_id_test() {
        // cargo test postgresql_user_repository_find_by_group_id_test -- --ignored --nocapture
        dotenv().ok();
        let settings = Settings::load();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&settings.postgresql_url)
            .await
            .expect("not connected");

        let mut repo = PostgresqlUserRepository::new(pool);

        let group_id =
            GroupId::from_uuid(Uuid::parse_str("8a20b54e-2df4-4bde-b00a-d344b12ac789").unwrap());

        let users = repo.find_by_group(&group_id).await.expect("error");

        for user in users {
            println!("{}", user.name());
        }
    }
}
