use std::{env as std_env, sync::LazyLock};

use dotenvy::dotenv;
use secrecy::Secret;

pub static JWT_SECRET: LazyLock<Secret<String>> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR)
        .expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    Secret::new(secret)
});

pub static DATABASE_URL: LazyLock<Secret<String>> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::DATABASE_URL_ENV_VAR)
        .expect("DATABASE_URL_ENV_VAR must be set.");
    if secret.is_empty() {
        panic!("DATABASE_URL_ENV_VAR must not be empty.");
    }
    Secret::new(secret)
});

pub static REDIS_HOST_NAME: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    std_env::var(env::REDIS_HOST_NAME_ENV_VAR).unwrap_or(DEFAULT_REDIS_HOSTNAME.to_owned())
});

pub static POSTMARK_AUTH_TOKEN: LazyLock<Secret<String>> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std_env::var(env::POSTMARK_AUTH_TOKEN_ENV_VAR)
        .expect("POSTMARK_AUTH_TOKEN must be set.");
    if secret.is_empty() {
        panic!("POSTMARK_AUTH_TOKEN must not be empty.");
    }
    Secret::new(secret)
});

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
    pub const POSTMARK_AUTH_TOKEN_ENV_VAR: &str = "POSTMARK_AUTH_TOKEN";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const PG_TABLE_NAME: &str = "users";
pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1"; 
pub const LOG_NAME: &str = "auth.log";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";

    pub mod email_client {
        use std::time::Duration;

        pub const BASE_URL: &str = "https://api.postmarkapp.com/email";
        pub const SENDER: &str = "bogdan@codeiron.io";
        pub const TIMEOUT: Duration = std::time::Duration::from_secs(10);
    }
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";

    pub mod email_client {
        use std::time::Duration;

        pub const SENDER: &str = "test@email.com";
        pub const TIMEOUT: Duration = std::time::Duration::from_millis(200);
    }
}