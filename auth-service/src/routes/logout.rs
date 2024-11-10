use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    app_state::app_state::AppState, 
    domain::error::AuthAPIError, 
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

#[tracing::instrument(name = "Logout", skip_all)]
pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let cookie = jar.get(JWT_COOKIE_NAME)
        .ok_or(AuthAPIError::MissingToken)?;

    let token = cookie.value().to_owned();
    validate_token(&token, state.token_store.clone()).await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    state.token_store
        .write()
        .await
        .add_token(&token)
        .await
        .map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;

    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    Ok((jar, StatusCode::OK))
}