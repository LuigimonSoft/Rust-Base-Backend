use std::env;

pub struct Config {
    pub port: u16,
    pub api_base: String,
    pub static_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3030".to_string())
                .parse()
                .expect("PORT must be a number"),
            api_base: env::var("API_BASE")
                .unwrap_or_else(|_| "/api/v1".trim_matches('/').to_string()),
            static_dir: env::var("STATIC_DIR").unwrap_or_else(|_| "public".to_string()),
        }
    }
}
