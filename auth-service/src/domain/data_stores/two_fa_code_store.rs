use rand::prelude::*;
use uuid::Uuid;

use crate::domain::email::Email;

#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(&mut self, email: Email, login_attempt_id: LoginAttemptId, code: TwoFACode) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        let s = Uuid::try_parse(&id)
            .map_err(|e| format!("failed parsing id: {} to uuid cause: {}", id, e))?;

        Ok(LoginAttemptId(s.to_string()))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        let id = Uuid::new_v4().to_string();
        LoginAttemptId(id)
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        match code.len() == 6 && code.chars().all(char::is_numeric) {
            true => Ok(Self(code)),
            false => Err(format!("failed parsing code: {}", code)),
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rnd: u32 = rng.gen_range(100_000..=999_999);
        Self(rnd.to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}