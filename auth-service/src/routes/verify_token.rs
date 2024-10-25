use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{
    domain::error::AuthAPIError, 
    utils::auth::validate_token, 
    app_state::app_state::AppState
};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    if request.token.is_empty() {
        return Err(AuthAPIError::MissingToken);
    }

    let token_store = state.token_store;

    let _res = validate_token(&request.token, token_store).await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    
    Ok(StatusCode::OK.into_response())
}