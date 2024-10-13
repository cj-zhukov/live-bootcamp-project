use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>
}

impl HashmapUserStore {
    pub fn new() -> Self {
        Self { users: HashMap::new() }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get_mut(&user.email) {
            Some(_) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email.clone(), user);
                Ok(())
            }
        } 
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(u) => Ok(u.clone()), // change return type to &user?
            None => Err(UserStoreError::UserNotFound)
        }
    }
    
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut map = HashmapUserStore::new();
        let user = User::new("foo", "bar", false);
        map.add_user(user.clone()).unwrap();
        let u = map.get_user(&user.email).unwrap();
        assert_eq!(u, user);
    }

    #[test]
    fn test_get_user() {
        let mut map = HashmapUserStore::new();
        let user = User::new("foo", "bar", false);
        map.add_user(user.clone()).unwrap();
        let u = map.get_user(&user.email).unwrap();
        assert_eq!(u, user);
    }

    #[test]
    fn test_validate_user() {
        let mut map = HashmapUserStore::new();
        let user = User::new("foo", "bar", false);
        map.add_user(user.clone()).unwrap();
        let res = map.validate_user(&user.email, &user.password).unwrap();
        assert_eq!(res, ());
    }
}