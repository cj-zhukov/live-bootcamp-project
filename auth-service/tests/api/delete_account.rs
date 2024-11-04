use auth_service::utils::constants::JWT_COOKIE_NAME;

use crate::helpers::TestApp;

// #[tokio::test]
// async fn should_return_200_if_valid_input() {
//     let app = TestApp::new().await;

//     // signup
//     let signup_body = serde_json::json!({
//         "email": "test@email.com",
//         "password": "password123",
//         "requires2FA": false
//     });
//     let response = app.post_signup(&signup_body).await;
//     assert_eq!(response.status().as_u16(), 201);

//     // login
//     let login_body = serde_json::json!({
//         "email": "test@email.com",
//         "password": "password123",
//     });
//     let response = app.post_login(&login_body).await;
//     assert_eq!(response.status().as_u16(), 200);

//     // delete account
//     let auth_cookie = response
//         .cookies()
//         .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
//         .expect("No auth cookie found");

//     let body = serde_json::json!({
//         "token": auth_cookie.value(),
//     });

//     let response = app.post_delete_account(&body).await;
//     assert_eq!(response.status().as_u16(), 200);

//     // signup again to check user was deleted
//     let signup_body = serde_json::json!({
//         "email": "test@email.com",
//         "password": "password123",
//         "requires2FA": false
//     });
//     let response = app.post_signup(&signup_body).await;
//     assert_eq!(response.status().as_u16(), 201);
// }