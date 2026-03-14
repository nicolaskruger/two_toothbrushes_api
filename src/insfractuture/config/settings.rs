use std::env;

use crate::insfractuture::config::dto::env_group::EnvGroup;

pub struct Settings {
    pub postgresql_url: String,
    pub group_list: Vec<EnvGroup>,
}

impl Settings {
    pub fn load() -> Self {
        Self {
            postgresql_url: env::var("DATABASE_URL").expect("not set DATABASE_URL"),
            group_list: env::var("GROUP_LIST")
                .map_err(|e| e.to_string())
                .and_then(|e| serde_json::from_str(&e).map_err(|e| e.to_string()))
                .expect("not set GROUP_LIST"),
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

        // Settings::load();
    }
}
