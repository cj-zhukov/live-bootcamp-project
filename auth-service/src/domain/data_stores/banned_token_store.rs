use color_eyre::eyre::{Report, Result};
use thiserror::Error;

// #[derive(Debug, PartialEq)]
// pub enum BannedTokenStoreError {
//     UnexpectedError,
// }

#[derive(Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}