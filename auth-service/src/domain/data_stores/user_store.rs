use color_eyre::eyre::Report;
use thiserror::Error;

use crate::domain::user::User;
use crate::domain::email::Email;
use crate::domain::password::Password;

#[async_trait::async_trait]
pub trait UserStore {
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn delete_user(&mut self, email: &Email) -> Result<(), UserStoreError>;
}

// #[derive(Debug, PartialEq)]
// pub enum UserStoreError {
//     UserAlreadyExists,
//     UserNotFound,
//     InvalidCredentials,
//     UnexpectedError,
// }

#[derive(Debug, Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::UserNotFound, Self::UserNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}