use crate::{
    app_state::app_state::AppState, 
    domain::{
        email::Email, 
        error::AuthAPIError, 
        password::Password
    },
    utils::auth::generate_auth_cookie,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = Email::parse(&request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let pwd = Password::parse(&request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;
    user_store.validate_user(&email, &pwd).await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    user_store.get_user(&email).await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let auth_cookie = generate_auth_cookie(&email)
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, StatusCode::OK.into_response()))
}