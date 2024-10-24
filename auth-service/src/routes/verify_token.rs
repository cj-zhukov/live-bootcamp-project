use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{domain::error::AuthAPIError, utils::auth::validate_token};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token(Json(request): Json<VerifyTokenRequest>,) -> Result<impl IntoResponse, AuthAPIError> {
    if request.token.is_empty() {
        return Err(AuthAPIError::MissingToken);
    }

    let _res = validate_token(&request.token).await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    
    Ok(StatusCode::OK.into_response())
}