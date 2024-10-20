use crate::{app_state::app_state::AppState, domain::{email::Email, error::AuthAPIError, password::Password}};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let pwd = Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;
    if let Err(_e) = user_store.validate_user(&email, &pwd).await {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    if let Err(_e) = user_store.get_user(&email).await {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let response = Json(LoginResponse {
        message: "User login successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginResponse {
    pub message: String,
}