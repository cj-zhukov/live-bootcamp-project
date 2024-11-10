// use crate::{
//     app_state::app_state::AppState, 
//     domain::{
//         data_stores::{LoginAttemptId, TwoFACode, UserStoreError}, email::Email, error::AuthAPIError, password::Password
//     },
//     utils::auth::generate_auth_cookie,
// };

// use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
// use axum_extra::extract::CookieJar;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize)]
// #[serde(untagged)]
// pub enum LoginResponse {
//     RegularAuth,
//     TwoFactorAuth(TwoFactorAuthResponse),
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TwoFactorAuthResponse {
//     pub message: String,
//     #[serde(rename = "loginAttemptId")]
//     pub login_attempt_id: String,
// }

// #[derive(Deserialize)]
// pub struct LoginRequest {
//     pub email: String,
//     pub password: String,
// }

// pub async fn login(
//     State(state): State<AppState>,
//     jar: CookieJar,
//     Json(request): Json<LoginRequest>,
// ) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
//     let email = Email::parse(&request.email)
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;

//     let pwd = Password::parse(&request.password)
//         .map_err(|_| AuthAPIError::InvalidCredentials)?;

//     let user_store = &state.user_store.read().await;
//     user_store.validate_user(&email, &pwd).await
//         .map_err(|_| AuthAPIError::IncorrectCredentials)?;

//     // let user = user_store.get_user(&email).await
//     //     .map_err(|_| AuthAPIError::IncorrectCredentials)?;
//     match user_store.get_user(&email).await {
//         Ok(user) => match user.requires_2fa {
//             true => handle_2fa(&email, &state, jar).await,
//             false => handle_no_2fa(&email, jar).await,
//         },
//         Err(e) => match e {
//             UserStoreError::UserNotFound | UserStoreError::InvalidCredentials => return Err(AuthAPIError::InvalidCredentials),
//             _ => return Err(AuthAPIError::UnexpectedError)
//         }
//     }

//     // match user.requires_2fa {
//     //     true => handle_2fa(&email, &state, jar).await,
//     //     false => handle_no_2fa(&email, jar).await,
//     // }
// }

// async fn handle_no_2fa(email: &Email, jar: CookieJar) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
//     let auth_cookie = generate_auth_cookie(&email)
//         .map_err(|_| AuthAPIError::UnexpectedError)?;
//     let updated_jar = jar.add(auth_cookie);
//     let response = Json(LoginResponse::RegularAuth);

//     Ok((updated_jar, (StatusCode::OK, response)))
// }

// async fn handle_2fa(email: &Email, state: &AppState, jar: CookieJar) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
//     let login_attempt_id = LoginAttemptId::default();
//     let two_fa_code = TwoFACode::default();

//     let mut code_store = state.two_fa_code_store.write().await;
//     code_store.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await
//         .map_err(|_| AuthAPIError::UnexpectedError)?;

//     let email_client = state.email_client.read().await;
//     email_client.send_email(email, "2fa subject", two_fa_code.as_ref()).await
//         .map_err(|_| AuthAPIError::UnexpectedError)?;

//     let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
//         message: "2FA required".to_owned(),
//         login_attempt_id: login_attempt_id.as_ref().to_string(),
//     }));

//     Ok((jar, (StatusCode::PARTIAL_CONTENT, response)))
// }