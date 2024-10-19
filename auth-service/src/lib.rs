pub mod routes;
pub mod services;
pub mod domain;
pub mod app_state;

use domain::data_stores::user_store::UserStore;
use crate::domain::error::AuthAPIError;

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    serve::Serve,
    Json, Router,
};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
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
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        
        (status, body).into_response()
    }
}