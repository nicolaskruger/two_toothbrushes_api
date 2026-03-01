use std::env;

pub struct Settings {
    pub postgresql_url: String,
}

impl Settings {
    pub fn load() -> Self {
        Self {
            postgresql_url: env::var("POSTGRESQL_URI").expect("not set POSTGRESQL_URI"),
        }
    }
}
