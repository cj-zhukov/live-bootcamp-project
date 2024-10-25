use auth_service::{domain::email::Email, utils::auth::generate_auth_cookie};

use crate::helpers::{TestApp, get_random_email};

// #[tokio::test]
// async fn should_return_200_valid_token() {
//     let app = TestApp::new().await;

//     // add valid token 
//     let email = get_random_email();
//     let email = Email::parse(&email).unwrap();
//     let token = generate_auth_cookie(&email).unwrap();
//     let token = token.value();
//     let input = serde_json::json!({
//         "token": token,
//     });

//     let response = app.post_verify_token(&input).await;
//     assert_eq!(response.status().as_u16(), 200);
// }

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let input = serde_json::json!({
        "token": "invalid_token",
    });

    let response = app.post_verify_token(&input).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    // mailformed input
    let input = serde_json::json!({
        "foo": "bar",
    });

    let response = app.post_verify_token(&input).await;
    assert_eq!(response.status().as_u16(), 422);
}