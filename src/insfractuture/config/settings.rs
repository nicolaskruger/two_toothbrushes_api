use std::env;

pub struct Settings {
    pub postgresql_url: String,
}

impl Settings {
    pub fn load() -> Self {
        Self {
            postgresql_url: env::var("DATABASE_URL").expect("not set DATABASE_URL"),
        }
    }
}
