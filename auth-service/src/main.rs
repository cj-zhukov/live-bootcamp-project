use std::sync::Arc;

use auth_service::{services::hashmap_user_store::HashmapUserStore, app_state::app_state::AppState, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::new();
    let user_store = Arc::new(RwLock::new(user_store));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}