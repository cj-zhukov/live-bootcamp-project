use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use serde::{Serialize, Deserialize};
use sqlx::PgPool;

use crate::{domain::{data_stores::{UserStore, UserStoreError}, email::Email, password::Password, user::User}, utils::{auth::validate_token, constants::PG_TABLE_NAME}};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Users {
    pub email: String,
    pub password_hash: String,
    pub requires_2fa: bool,
}

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let task = tokio::task::spawn_blocking(move || {
            compute_password_hash(user.password.as_ref().to_string())
                .map_err(|_| UserStoreError::InvalidCredentials)
        });
        let task = task.await.map_err(|_| UserStoreError::UnexpectedError)?;
        let password = task.map_err(|_| UserStoreError::InvalidCredentials)?;

        let sql = format!("insert into {} (email, password_hash, requires_2fa) values ($1, $2, $3)", PG_TABLE_NAME);
        let query = sqlx::query(&sql);
        query
            .bind(user.email.as_ref())
            .bind(password)
            .bind(user.requires_2fa)
            .execute(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let sql = format!("select * from {} where email = $1", PG_TABLE_NAME);
        let query = sqlx::query_as::<_, Users>(&sql);
        let data = match query.bind(email.as_ref()).fetch_one(&self.pool).await {
            Ok(u) => u,
            Err(e) => match e {
                sqlx::Error::RowNotFound => return Err(UserStoreError::UserNotFound),
                _ => return Err(UserStoreError::UnexpectedError),
            }
        };

        let email = Email::parse(&data.email)
            .map_err(|_| UserStoreError::InvalidCredentials)?;
        let password = Password::parse(&data.password_hash)
            .map_err(|_| UserStoreError::InvalidCredentials)?;
        let user = User::new(email, password, data.requires_2fa);

        Ok(user)
    }
    
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        let sql = format!("select * from {} where email = $1", PG_TABLE_NAME);
        let query = sqlx::query_as::<_, Users>(&sql);
        let data = match query.bind(email.as_ref()).fetch_one(&self.pool).await {
            Ok(u) => u,
            Err(e) => match e {
                sqlx::Error::RowNotFound => return Err(UserStoreError::UserNotFound),
                _ => return Err(UserStoreError::UnexpectedError),
            }
        };

        let pwd_hash = data.password_hash;
        let pwd = password.as_ref().to_string();
        let task = tokio::task::spawn_blocking(|| {
            verify_password_hash(pwd_hash, pwd)
                .map_err(|_| UserStoreError::InvalidCredentials)
        });
        let task = task.await.map_err(|_| UserStoreError::UnexpectedError)?;

        if let Err(_) = task {    
            return Err(UserStoreError::InvalidCredentials);
        }
            
        Ok(())
    }

    async fn delete_user(&mut self, email: &Email) -> Result<(), UserStoreError> {
        // match self.users.remove(email) {
        //     Some(_) => Ok(()),
        //     None => Err(UserStoreError::UserNotFound)
        // }
        todo!()
    }
}

// Helper function to verify if a given password matches an expected hash
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error>> {
    let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_password_hash)?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .map_err(|e| e.into())
}

// Helper function to hash passwords before persisting them in the database.
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
fn compute_password_hash(password: String) -> Result<String, Box<dyn Error>> {
    let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?,
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

    Ok(password_hash)
}