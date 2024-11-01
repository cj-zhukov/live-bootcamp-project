use std::collections::HashMap;

use crate::domain::{
    data_stores::two_fa_code_store::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(&mut self, email: Email, login_attempt_id: LoginAttemptId, code: TwoFACode) -> Result<(), TwoFACodeStoreError> {
        let _res = self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let _ = self.codes.remove(&email)
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        Ok(())
        // match self.codes.remove(email) {
        //     Some(_) => Ok(()),
        //     None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        // }
    }

    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let res = self.codes.get(&email)
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        Ok(res.to_owned())
        // match self.codes.get(email) {
        //     Some(value) => Ok(value.clone()),
        //     None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut code_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@email.com").unwrap();
        let login_attempt_id = LoginAttemptId::parse("550e8400-e29b-41d4-a716-446655440000".to_string()).unwrap();
        let code = TwoFACode::parse("123456".to_string()).unwrap();

        let res = code_store.add_code(email, login_attempt_id, code).await;
        assert_eq!(res, Ok(()));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut code_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@email.com").unwrap();
        let login_attempt_id = LoginAttemptId::parse("550e8400-e29b-41d4-a716-446655440000".to_string()).unwrap();
        let code = TwoFACode::parse("123456".to_string()).unwrap();
        let res = code_store.add_code(email.clone(), login_attempt_id, code).await;
        assert_eq!(res, Ok(()));

        // remove code that exists
        let res = code_store.remove_code(&email).await;
        assert_eq!(res, Ok(()));

        // remove code that doesn't exist
        let email = Email::parse("email_doesnot_exist@com").unwrap();
        let res = code_store.remove_code(&email).await;
        assert_eq!(res, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut code_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@email.com").unwrap();
        let login_attempt_id = LoginAttemptId::parse("550e8400-e29b-41d4-a716-446655440000".to_string()).unwrap();
        let code = TwoFACode::parse("123456".to_string()).unwrap();
        let res = code_store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await;
        assert_eq!(res, Ok(()));

        // get code that exists
        let res = code_store.get_code(&email).await;
        assert_eq!(res, Ok((login_attempt_id, code)));

        // get code that doesn't exist
        let email = Email::parse("email_doesnot_exist@com").unwrap();
        let res = code_store.remove_code(&email).await;
        assert_eq!(res, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));
    }
}