use crate::helpers::TestApp;
use auth_service::{routes::LoginRequest, ErrorResponse};

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let input = [
        serde_json::json!({
            "email": "email",
            "password": "foo",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "email",
            "password": "bar",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "email",
            "password": "baz",
            "requires2FA": true
        }),
    ];

    for i in input.iter() {
        let response = app.post_signup(i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    // test mailformed creds
    let test_case = serde_json::json!({
        "email": "email@com",
    });

    let response = app.post_login(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        422,
        "Failed for input: {:?}",
        test_case
    );
}