use crate::helpers::{TestApp, get_random_email};

// use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

// #[tokio::test]
// async fn should_return_200_valid_token() {
//     let app = TestApp::new().await;

//     // how to add valid token 
//     let input = serde_json::json!({
//         "token": "foo",
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