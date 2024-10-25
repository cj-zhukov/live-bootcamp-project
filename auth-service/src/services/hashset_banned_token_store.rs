use std::collections::{HashMap, HashSet};

use crate::domain::data_stores::banned_token_store::{BannedTokenStore, BannedTokenStoreError};  

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
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        if !self.token_exists(token).await {
            let _res = self.tokens.insert(token.to_string());
        }
        Ok(())
    }

    async fn token_exists(&self, token: &str) -> bool {
        self.tokens.contains(token)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut map = HashsetBannedTokenStore::new();
        map.add_token("foo").await.unwrap();
        let res = map.tokens.get("foo");
        assert_eq!(res, Some(&"foo".to_string()));
    }

    #[tokio::test]
    async fn test_token_exists() {
        let mut map = HashsetBannedTokenStore::new();
        map.add_token("foo").await.unwrap();
        let res = map.token_exists("foo").await;
        assert_eq!(res, true);

        let res = map.token_exists("bar").await;
        assert_eq!(res, false);
    }
}