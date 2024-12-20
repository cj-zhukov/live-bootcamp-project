use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use secrecy::Secret;
use serde::Deserialize;

use crate::{
    domain::error::AuthAPIError, 
    utils::auth::validate_token, 
    app_state::app_state::AppState
};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: Secret<String>,
}

#[tracing::instrument(name = "verify_token", skip_all)]
pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match validate_token(&request.token, state.token_store.clone()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}