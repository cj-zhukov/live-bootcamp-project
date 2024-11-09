use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        let key = get_key(token);
        let value = true;

        let secs: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        let _: () = self
            .conn
            .write()
            .await
            .set_ex(key, value, secs)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let key = get_key(token);

        let is_banned: bool = self
            .conn
            .write()
            .await
            .exists(key)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(is_banned)
    }
}

const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}