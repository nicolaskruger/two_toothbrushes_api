use sqlx::postgres::PgPoolOptions;

use crate::insfractuture::config::settings::Settings;

pub async fn migrate() -> Result<(), sqlx::migrate::MigrateError> {
    let settings = Settings::load();

    let pool = PgPoolOptions::new()
        .connect(&settings.postgresql_url)
        .await?;

    sqlx::migrate!().run(&pool).await
}
