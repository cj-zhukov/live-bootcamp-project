use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::app_state::AppState;
use crate::domain::{email::Email, error::AuthAPIError, password::Password, user::User};

pub async fn delete_account(
    State(state): State<AppState>,
    Json(request): Json<DeleteRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let pwd = Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email.clone(), pwd, request.requires_2fa);
    
    let mut user_store = state.user_store.write().await;
    if let Err(_e) = user_store.delete_user(user).await {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(DeleteResponse {
        message: "User deleted successfully!".to_string(),
    });

    Ok((StatusCode::OK, response))
}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteResponse {
    pub message: String,
}