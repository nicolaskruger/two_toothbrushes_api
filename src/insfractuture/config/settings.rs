use std::env;

use chrono::{DateTime, Utc};

use crate::insfractuture::config::dto::env_group::EnvGroup;

pub struct Settings {
    pub postgresql_url: String,
    pub group_list: Vec<EnvGroup>,
    pub auth_secret: String,
    pub payment_token: String,
    pub confirm_limit: DateTime<Utc>,
    pub api_key: String,
}

impl Settings {
    pub fn load() -> Self {
        Self {
            postgresql_url: env::var("DATABASE_URL").expect("not set DATABASE_URL"),
            group_list: env::var("GROUP_LIST")
                .map_err(|e| e.to_string())
                .and_then(|e| serde_json::from_str(&e).map_err(|e| e.to_string()))
                .expect("not set GROUP_LIST"),
            auth_secret: env::var("AUTH_SECRET").expect("not set AUTH_SECRET"),
            payment_token: env::var("PAYMENT_TOKEN").expect("not set PAYMENT_TOKEN"),
            confirm_limit: env::var("CONFIRM_LIMIT")
                .expect("not set CONFIRM_LIMIT")
                .parse::<DateTime<Utc>>()
                .expect("CONFIRM_LIMIT must be a valid RFC3339 datetime"),
            api_key: env::var("API_KEY").expect("not set API_KEY"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::insfractuture::config::settings::Settings;

    use dotenv::dotenv;
    #[tokio::test]
    #[ignore = "env test"]
    async fn load_env() {
        // cargo test load_env -- --ignored --nocapture

        dotenv().expect("something whet wrong loading env variables");

        Settings::load();
    }
}
