use std::sync::Arc;

use tokio::sync::RwLock;

use auth_service::{
    app_state::app_state::AppState,
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, 
        hashmap_user_store::HashmapUserStore, 
        hashset_banned_token_store::HashsetBannedTokenStore
    }, 
    utils::constants::prod, 
    Application
};

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::new();
    let user_store = Arc::new(RwLock::new(user_store));
    let token_store = HashsetBannedTokenStore::new();
    let token_store = Arc::new(RwLock::new(token_store));
    let two_fa_code_store = HashmapTwoFACodeStore::default();
    let two_fa_code_store = Arc::new(RwLock::new(two_fa_code_store));

    let app_state = AppState::new(user_store, token_store, two_fa_code_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}