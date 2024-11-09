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
        let password = compute_password_hash(user.password.as_ref().to_owned())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        let sql = format!("insert into {} (email, password_hash, requires_2fa) values ($1, $2, $3)", PG_TABLE_NAME);
        sqlx::query(&sql)
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
        query
            .bind(email.as_ref())
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?
            .map(|u| {
                let email = Email::parse(&u.email).map_err(|_| UserStoreError::InvalidCredentials)?;
                let password = Password::parse(&u.password_hash).map_err(|_| UserStoreError::InvalidCredentials)?;
                Ok(User::new(email, password, u.requires_2fa))
            })
            .ok_or(UserStoreError::UserNotFound)?
    }
    
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        verify_password_hash(
            user.password.as_ref().to_owned(),
            password.as_ref().to_owned(),
        )
        .await
        .map_err(|_| UserStoreError::InvalidCredentials)
    }

    async fn delete_user(&mut self, email: &Email) -> Result<(), UserStoreError> {
        todo!()
    }
}

async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let res = tokio::task::spawn_blocking(move || {
        let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_password_hash)?;

        Argon2::default()
            .verify_password(password_candidate.as_bytes(), &expected_password_hash)
            .map_err(|e| e.into())
    })
    .await;

    res?
}

async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let res = tokio::task::spawn_blocking(move || {
        let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

        Ok(password_hash)
    })
    .await;

    res?
}