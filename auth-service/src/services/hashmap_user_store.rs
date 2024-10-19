use std::collections::HashMap;

use crate::domain::data_stores::user_store::{UserStore, UserStoreError};
use crate::domain::password::Password;
use crate::domain::user::User;
use crate::domain::email::Email;


#[derive(Default)]
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
        let user = self.get_user(email).await?;
        if user.password == *password {
            Ok(())
        } else {
            Err(UserStoreError::UserNotFound)
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
        map.add_user(user.clone()).await.unwrap();
        let u = map.get_user(&user.email).await.unwrap();
        assert_eq!(u, user);
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut map = HashmapUserStore::new();
        let email = Email::parse("foo@com").unwrap();
        let pwd = Password::parse("foobarbaz").unwrap();
        let user = User::new(email, pwd, false);
        map.add_user(user.clone()).await.unwrap();
        let u = map.get_user(&user.email).await.unwrap();
        assert_eq!(u, user);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut map = HashmapUserStore::new();
        let email = Email::parse("foo@com").unwrap();
        let pwd = Password::parse("foobarbaz").unwrap();
        let user = User::new(email, pwd, false);
        map.add_user(user.clone()).await.unwrap();
        let res = map.validate_user(&user.email, &user.password).await.unwrap();
        assert_eq!(res, ());
    }
}