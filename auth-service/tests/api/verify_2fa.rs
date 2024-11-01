use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_200_if_correct_code() {
    todo!()
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    todo!()
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    todo!()
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail. 
    todo!()
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {    
    todo!()
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    todo!()
}