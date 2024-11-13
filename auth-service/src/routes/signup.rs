use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::app_state::app_state::AppState;
use crate::domain::{email::Email, error::AuthAPIError, password::Password, user::User, data_stores::UserStoreError};
use secrecy::Secret;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: Secret<String>,
    pub password: Secret<String>,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignupResponse {
    pub message: String,
}

#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let pwd = Password::parse(request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email.clone(), pwd, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    match user_store.get_user(&email).await {
        Ok(_u) => return Err(AuthAPIError::UserAlreadyExists),
        Err(e) => match e {
            UserStoreError::UserAlreadyExists => return Err(AuthAPIError::UserAlreadyExists),
            UserStoreError::InvalidCredentials => return Err(AuthAPIError::InvalidCredentials),
            UserStoreError::UnexpectedError(e) => return Err(AuthAPIError::UnexpectedError(e.into())),
            UserStoreError::UserNotFound => {
                user_store.add_user(user).await
                    .map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;
        
                let response = Json(SignupResponse {
                    message: "User created successfully!".to_string(),
                });
            
                Ok((StatusCode::CREATED, response))
            }
        }
    }

    // if let Err(e) = user_store.add_user(user).await {
    //     return Err(AuthAPIError::UnexpectedError(e.into()));
    // }

    // let response = Json(SignupResponse {
    //     message: "User created successfully!".to_string(),
    // });

    // Ok((StatusCode::CREATED, response))
}