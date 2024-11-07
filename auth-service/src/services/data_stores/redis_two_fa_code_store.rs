use std::sync::Arc;

use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    async fn add_code(&mut self, email: Email, login_attempt_id: LoginAttemptId, code: TwoFACode) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(&email);
        let two_fa_tuple = TwoFATuple(login_attempt_id.as_ref().to_string(), code.as_ref().to_string());
        let two_fa_tuple = serde_json::to_string(&two_fa_tuple).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        let mut conn = self.conn.write().await;
        let secs: u64 = TEN_MINUTES_IN_SECONDS.try_into().map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        let _res = conn.set_ex(key, two_fa_tuple, secs).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(email);
        let mut conn = self.conn.write().await;
        let _res = conn.del(key).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let key = get_key(email);
        let mut conn = self.conn.write().await;
        let two_fa_tuple: String = conn.get(key).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        let two_fa_tuple: TwoFATuple = serde_json::from_str(&two_fa_tuple).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        let login_attempt_id = LoginAttemptId::parse(two_fa_tuple.0).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        let two_fa_code = TwoFACode::parse(two_fa_tuple.1).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        Ok((login_attempt_id, two_fa_code))
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}