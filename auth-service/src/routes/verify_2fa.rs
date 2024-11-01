use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::app_state::AppState, 
    domain::{
        data_stores::two_fa_code_store::{LoginAttemptId, TwoFACode}, 
        email::Email, error::AuthAPIError
    }, utils::auth::generate_auth_cookie
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    login_attempt_id: String,
    two_fa_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verify2FAResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

// pub async fn verify_2fa() -> impl IntoResponse {
//     StatusCode::OK.into_response()
// }

// pub async fn verify_2fa(
//     State(state): State<AppState>,
//     jar: CookieJar,
//     Json(request): Json<Verify2FARequest>,
// ) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
//     let email = Email::parse(&request.email)
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;

//     let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id)
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;

//     let two_fa_code = TwoFACode::parse(request.two_fa_code)
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;

//     let mut two_fa_code_store = state.two_fa_code_store.write().await;
//     let (login_attempt_id_res, two_fa_code_res) = two_fa_code_store.get_code(&email).await
//         .map_err(|_| AuthAPIError::IncorrectCredentials)?;

//     if login_attempt_id.as_ref() != login_attempt_id_res.as_ref() || two_fa_code.as_ref() != two_fa_code_res.as_ref() {
//         return Err(AuthAPIError::IncorrectCredentials);
//     }

//     two_fa_code_store.remove_code(&email).await
//         .map_err(|_| AuthAPIError::UnexpectedError)?;

//     let auth_cookie = generate_auth_cookie(&email)
//         .map_err(|_| AuthAPIError::UnexpectedError)?;
//     let updated_jar = jar.add(auth_cookie);
//     // two_fa_code_store.remove_code(&email).await
//     //     .map_err(|_| AuthAPIError::UnexpectedError)?;

//     Ok((updated_jar, StatusCode::OK))
// }

pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(&request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id.clone()) {
        Ok(login_attempt_id) => login_attempt_id,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(two_fa_code) => two_fa_code,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    let code_tuple = match two_fa_code_store.get_code(&email).await {
        Ok(code_tuple) => code_tuple,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    if !code_tuple.0.eq(&login_attempt_id) || !code_tuple.1.eq(&two_fa_code) {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    if two_fa_code_store.remove_code(&email).await.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    let cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(cookie);

    (updated_jar, Ok(()))
}