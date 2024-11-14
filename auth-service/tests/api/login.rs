use secrecy::{ExposeSecret, Secret};
use wiremock::{matchers::{method, path}, Mock, ResponseTemplate};

use crate::helpers::{get_random_email, TestApp};
use auth_service::{domain::email::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME, ErrorResponse};

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // -------------------------------------------
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // -------------------------------------------
    // Define an expectation for the mock server
    Mock::given(path("/email")) // Expect an HTTP request to the "/email" path
        .and(method("POST")) // Expect the HTTP method to be POST
        .respond_with(ResponseTemplate::new(200)) // Respond with an HTTP 200 OK status
        .expect(1) // Expect this request to be made exactly once
        .mount(&app.email_server) // Mount this expectation on the mock email server
        .await; // Await the asynchronous operation to ensure the mock server is set up before proceeding

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let two_fa_code_store = app.two_fa_code_store.clone();
    let two_fa_code_store = two_fa_code_store.read().await;

    let code_tuple = two_fa_code_store
        .get_code(&Email::parse(Secret::new(random_email)).unwrap())
        .await
        .expect("Failed to get 2FA code");

    assert_eq!(code_tuple.0.as_ref().expose_secret().clone(), json_body.login_attempt_id);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let test_cases = vec![
        ("invalid_email", "password123"),
        (random_email.as_str(), "invalid"),
        ("", "password123"),
        (random_email.as_str(), ""),
        ("", ""),
    ];

    for (email, password) in test_cases {
        let login_body = serde_json::json!({
            "email": email,
            "password": password
        });
        let response = app.post_login(&login_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            login_body
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // -------------------------------------------

    let test_cases = vec![
        (random_email.as_str(), "wrong-password"),
        ("wrong@email.com", "password123"),
        ("wrong@email.com", "wrong-password"),
    ];

    for (email, password) in test_cases {  
        // Mock::given(path("/email"))
        //     .and(method("POST"))
        //     .respond_with(ResponseTemplate::new(200))
        //     .expect(1)
        //     .mount(&app.email_server)
        //     .await;

        let login_body = serde_json::json!({
            "email": email,
            "password": password
        });
        let response = app.post_login(&login_body).await;

        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            login_body
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Incorrect credentials".to_owned()
        );
    }

    app.clean_up().await;
}

// #[tokio::test]
// async fn should_return_401_if_old_code() {
//     let mut app = TestApp::new().await;

//     let random_email = get_random_email();

//     let signup_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//         "requires2FA": false
//     });

//     let response = app.post_signup(&signup_body).await;
//     assert_eq!(response.status().as_u16(), 201);

//     // -------------------------------------------

//     Mock::given(path("/email"))
//         .and(method("POST"))
//         .respond_with(ResponseTemplate::new(200))
//         .expect(1)
//         .mount(&app.email_server)
//         .await;

//     let test_cases = vec![
//         (random_email.as_str(), "wrong-password"),
//         ("wrong@email.com", "password123"),
//         ("wrong@email.com", "wrong-password"),
//     ];

//     for (email, password) in test_cases {
//         let login_body = serde_json::json!({
//             "email": email,
//             "password": password
//         });
//         let response = app.post_login(&login_body).await;

//         assert_eq!(
//             response.status().as_u16(),
//             401,
//             "Failed for input: {:?}",
//             login_body
//         );

//         assert_eq!(
//             response
//                 .json::<ErrorResponse>()
//                 .await
//                 .expect("Could not deserialize response body to ErrorResponse")
//                 .error,
//             "Incorrect credentials".to_owned()
//         );
//     }

//     app.clean_up().await;
// }

// #[tokio::test]
// async fn should_return_401_if_same_code_twice() {
//     let mut app = TestApp::new().await;

//     let random_email = get_random_email();

//     let signup_body = serde_json::json!({
//         "email": random_email,
//         "password": "password123",
//         "requires2FA": false
//     });

//     let response = app.post_signup(&signup_body).await;
//     assert_eq!(response.status().as_u16(), 201);

//     // -------------------------------------------

//     Mock::given(path("/email"))
//         .and(method("POST"))
//         .respond_with(ResponseTemplate::new(200))
//         .expect(1)
//         .mount(&app.email_server)
//         .await;

//     let test_cases = vec![
//         (random_email.as_str(), "wrong-password"),
//         ("wrong@email.com", "password123"),
//         ("wrong@email.com", "wrong-password"),
//     ];

//     for (email, password) in test_cases {
//         let login_body = serde_json::json!({
//             "email": email,
//             "password": password
//         });
//         let response = app.post_login(&login_body).await;

//         assert_eq!(
//             response.status().as_u16(),
//             401,
//             "Failed for input: {:?}",
//             login_body
//         );

//         assert_eq!(
//             response
//                 .json::<ErrorResponse>()
//                 .await
//                 .expect("Could not deserialize response body to ErrorResponse")
//                 .error,
//             "Incorrect credentials".to_owned()
//         );
//     }

//     app.clean_up().await;
// }

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let test_cases = [
        serde_json::json!({
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }

    app.clean_up().await;
}