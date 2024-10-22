use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::app_state::AppState;
use crate::domain::{email::Email, error::AuthAPIError};

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteResponse {
    pub message: String,
}

pub async fn delete_account(
    State(state): State<AppState>,
    Json(request): Json<DeleteRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let mut user_store = state.user_store.write().await;
    if let Err(_e) = user_store.delete_user(&email).await {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(DeleteResponse {
        message: "User deleted successfully!".to_string(),
    });

    Ok((StatusCode::OK, response))
}