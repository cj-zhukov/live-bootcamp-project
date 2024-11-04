use std::collections::HashMap;

use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::password::Password;
use crate::domain::user::User;
use crate::domain::email::Email;

#[derive(Default, Debug)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>
}

impl HashmapUserStore {
    pub fn new() -> Self {
        Self { users: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get_mut(&user.email) {
            Some(_) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email.clone(), user);
                Ok(())
            }
        } 
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(u) => Ok(u.to_owned()),
            None => Err(UserStoreError::UserNotFound)
        }
    }
    
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn delete_user(&mut self, email: &Email) -> Result<(), UserStoreError> {
        match self.users.remove(email) {
            Some(_) => Ok(()),
            None => Err(UserStoreError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut map = HashmapUserStore::new();
        let email = Email::parse("foo@com").unwrap();
        let pwd = Password::parse("foobarbaz").unwrap();
        let user = User::new(email, pwd, false);

        // Test adding a new user
        let res = map.add_user(user.clone()).await;
        assert!(res.is_ok());

        // Test adding an existing user
        let res = map.add_user(user).await;
        assert_eq!(res, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut map = HashmapUserStore::new();
        let email = Email::parse("foo@com").unwrap();
        let pwd = Password::parse("foobarbaz").unwrap();
        let user = User::new(email.clone(), pwd, false);

        // Test getting a user that exists
        map.users.insert(email.clone(), user.clone());
        let res = map.get_user(&email).await;
        assert_eq!(res, Ok(user));

        // Test getting a user that doesn't exist
        let result = map
            .get_user(&Email::parse("nonexistent@example.com").unwrap())
            .await;
   
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut map = HashmapUserStore::new();
        let email = Email::parse("foo@com").unwrap();
        let pwd = Password::parse("foobarbaz").unwrap();
        let user = User::new(email.clone(), pwd.clone(), false);

        // Test validating a user that exists with correct password
        map.users.insert(email.clone(), user.clone());
        let res = map.validate_user(&email, &pwd).await;
        assert_eq!(res, Ok(()));

        // Test validating a user that exists with incorrect password
        let wrong_password = Password::parse("wrongpassword").unwrap();
        let res = map.validate_user(&email, &wrong_password).await;
        assert_eq!(res, Err(UserStoreError::InvalidCredentials));

        // Test validating a user that doesn't exist
        let res = map
            .validate_user(
                &Email::parse("nonexistent@example.com").unwrap(),
                &pwd,
            )
            .await;
   
        assert_eq!(res, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_delete_user() {
        let mut map = HashmapUserStore::new();
        let email = Email::parse("foo@com").unwrap();
        let pwd = Password::parse("foobarbaz").unwrap();
        let user = User::new(email.clone(), pwd, false);

        // Test getting a user that exists
        map.users.insert(email.clone(), user.clone());
        let res = map.get_user(&email).await;
        assert_eq!(res, Ok(user.clone()));

        // Deleting user
        map.delete_user(&email).await.unwrap();

        // Test getting a user that doesn't exist
        let res = map.get_user(&user.email).await;
        assert_eq!(res, Err(UserStoreError::UserNotFound));
    }
}