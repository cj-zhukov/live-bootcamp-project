use std::collections::HashSet;

use secrecy::{ExposeSecret, Secret};

use crate::domain::data_stores::{BannedTokenStore, BannedTokenStoreError};  

#[derive(Default, Clone)]
pub struct HashsetBannedTokenStore {
    pub tokens: HashSet<String>
}

impl HashsetBannedTokenStore {
    pub fn new() -> Self {
        Self { tokens: HashSet::new() }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token.expose_secret().to_string());
        Ok(())
    }

    async fn contains_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token.expose_secret()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashsetBannedTokenStore::new();
        let token = "test_token".to_owned();

        let res = store.add_token(Secret::new(token.clone())).await;

        assert!(res.is_ok());
        assert!(store.tokens.contains(&token));
    }

    #[tokio::test]
    async fn test_contains_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "test_token".to_owned();
        store.tokens.insert(token.clone());

        let res = store.contains_token(&Secret::new(token.clone())).await;

        assert!(res.is_ok());
        assert!(store.tokens.contains(&token));
    }
}