use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    domain::error::AuthAPIError, 
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    let _claims = match validate_token(&token).await {
        Ok(claims) => claims,
        Err(_e) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    // Remove JWT cookie from the CookieJar
    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(StatusCode::OK))
}