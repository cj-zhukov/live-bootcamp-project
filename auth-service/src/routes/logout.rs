use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    app_state::app_state::AppState, 
    domain::error::AuthAPIError, 
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();
    let token_store = state.token_store;

    let _claims = match validate_token(&token, token_store).await {
        Ok(claims) => claims,
        Err(_e) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    // Remove JWT cookie from the CookieJar
    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(StatusCode::OK))
}