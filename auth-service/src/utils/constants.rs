use dotenvy::dotenv;
use std::{env as std_env, sync::LazyLock};

pub static JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR)
        .expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
});

pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::DATABASE_URL_ENV_VAR)
        .expect("DATABASE_URL_ENV_VAR must be set.");
    if secret.is_empty() {
        panic!("DATABASE_URL_ENV_VAR must not be empty.");
    }
    secret
});

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const PG_TABLE_NAME: &str = "users";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}