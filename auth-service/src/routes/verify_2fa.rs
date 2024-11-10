use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::app_state::AppState, 
    domain::{
        data_stores::{LoginAttemptId, TwoFACode}, 
        email::Email, error::AuthAPIError
    }, utils::auth::generate_auth_cookie
};

#[derive(Serialize, Clone, Debug, PartialEq, Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FAResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

#[tracing::instrument(name = "verify_2fa", skip_all)]
pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let two_fa_code = TwoFACode::parse(request.two_fa_code)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    let (login_attempt_id_res, two_fa_code_res) = two_fa_code_store.get_code(&email).await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if login_attempt_id.as_ref() != login_attempt_id_res.as_ref() || two_fa_code.as_ref() != two_fa_code_res.as_ref() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let cookie = generate_auth_cookie(&email)
        .map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;
    let updated_jar = jar.add(cookie);

    two_fa_code_store.remove_code(&email).await
        .map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;

    Ok((updated_jar, StatusCode::OK))
}