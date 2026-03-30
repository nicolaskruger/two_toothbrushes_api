use sqlx::{PgPool, query};

use crate::domain::{
    repository::pix_repository::{PixError, PixRepository},
    value_object::pix_status::PixStatus,
};

pub struct PostgresPixRepository {
    pool: PgPool,
}

impl PostgresPixRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn _create_pix(
        &mut self,
        pix: crate::domain::value_object::pix::Pix,
    ) -> Result<crate::domain::value_object::pix::Pix, sqlx::Error> {
        query!(
            r#"
                INSERT INTO pix (
                    id,
                    amount,
                    qr_code,
                    qr_code_base64,
                    group_id,
                    status,
                    created_at
                ) VALUES (
                    $1, 
                    $2, 
                    $3, 
                    $4, 
                    $5, 
                    $6, 
                    $7  
                );
            "#,
            pix.id().as_uuid(),
            pix.amount(),
            pix.qr_code(),
            pix.qr_code_base64(),
            pix.group_id().as_uuid(),
            pix.status() as PixStatus,
            pix.created_at()
        )
        .execute(&self.pool)
        .await?;

        Ok(pix)
    }
}

impl PixRepository for PostgresPixRepository {
    async fn create_pix(
        &mut self,
        pix: crate::domain::value_object::pix::Pix,
    ) -> Result<
        crate::domain::value_object::pix::Pix,
        crate::domain::repository::pix_repository::PixError,
    > {
        let pix = self
            ._create_pix(pix)
            .await
            .map_err(|_| PixError::OnCreateError)?;

        Ok(pix)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;

    use crate::{
        domain::value_object::{group_id::GroupId, pix::Pix, pix_id::PixId},
        insfractuture::config::settings::Settings,
    };

    use super::*;

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_pix_repository_create_test() {
        // cargo test postgresql_group_repository_count_test -- --ignored
        dotenv().ok();
        let settings = Settings::load();

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&settings.postgresql_url)
            .await
            .expect("not connected");

        let mut repo = PostgresPixRepository::new(pool);

        let amount = 1.;
        let qr_code = "".to_string();
        let qr_code_base64 = "".to_string();
        let id = PixId::new();
        let status = PixStatus::Pending;
        let created_at = Utc::now();
        let group_id = GroupId::from_uuid(
            uuid::Uuid::parse_str("5e2cfced-b3d8-449d-a55c-1659968bbb70").unwrap(),
        );

        let pix = Pix::new(
            amount,
            qr_code,
            qr_code_base64,
            group_id,
            id,
            status,
            created_at,
        );

        let new_pix = repo._create_pix(pix.clone()).await.unwrap();

        assert_eq!(pix.clone().id().as_uuid(), new_pix.id().as_uuid())
    }
}
