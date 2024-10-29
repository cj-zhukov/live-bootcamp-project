use std::sync::Arc;

use reqwest::cookie::Jar;
use tokio::sync::RwLock;
use uuid::Uuid;

use auth_service::{
    app_state::app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType}, 
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, 
        hashmap_user_store::HashmapUserStore, 
        hashset_banned_token_store::HashsetBannedTokenStore, mock_email_client::MockEmailClient,
    }, 
    utils::constants::test, 
    Application, 
};

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = HashmapUserStore::new();
        let user_store = Arc::new(RwLock::new(user_store));
        let token_store = HashsetBannedTokenStore::new();
        let token_store = Arc::new(RwLock::new(token_store.clone()));
        let two_fa_code_store = HashmapTwoFACodeStore::default();
        let two_fa_code_store = Arc::new(RwLock::new(two_fa_code_store));
        let email_client = MockEmailClient;
        let email_client = Arc::new(RwLock::new(email_client));

        let app_state = AppState::new(user_store, token_store.clone(), two_fa_code_store.clone(), email_client);
        
        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self { address, cookie_jar, http_client, token_store, two_fa_code_store }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where 
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_delete_account<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/delete-account", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}