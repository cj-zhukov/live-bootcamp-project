pub mod routes;
pub mod services;
pub mod domain;
pub mod app_state;

use app_state::app_state::AppState;
use domain::data_stores::user_store::UserStore;
use domain::error::AuthAPIError;
use routes::*;

use std::error::Error;
use std::sync::Arc;

use axum::{
    http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post}, serve::Serve, Json, Router
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
    }
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/foo", get(|| async { "ok" }))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .route("/delete-account", delete(delete_account))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application::new(server, address))
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            Self::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            Self::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            },
            Self::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Unauthorized")
        };
        
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        
        (status, body).into_response()
    }
}