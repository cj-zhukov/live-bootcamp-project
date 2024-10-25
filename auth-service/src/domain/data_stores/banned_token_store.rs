#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError>;
    async fn token_exists(&self, token: &str) -> bool;
}