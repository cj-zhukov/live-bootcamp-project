use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::RwLock;

use auth_service::{
    app_state::app_state::AppState, get_postgres_pool, 
    services::data_stores::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, MockEmailClient, PostgresUserStore}, 
    utils::constants::{prod, DATABASE_URL}, 
    Application
};

#[tokio::main]
async fn main() {
    // let user_store = HashmapUserStore::new();
    // let user_store = Arc::new(RwLock::new(user_store));
    let pg_pool = configure_postgresql().await;
    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let token_store = HashsetBannedTokenStore::new();
    let token_store = Arc::new(RwLock::new(token_store));
    let two_fa_code_store = HashmapTwoFACodeStore::default();
    let two_fa_code_store = Arc::new(RwLock::new(two_fa_code_store));
    let email_client = Arc::new(RwLock::new(MockEmailClient));

    let app_state = AppState::new(user_store, token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database! 
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}