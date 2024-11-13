use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use color_eyre::eyre::{eyre, Context, Result};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use secrecy::{ExposeSecret, Secret};

use crate::{domain::{data_stores::{UserStore, UserStoreError}, email::Email, password::Password, user::User}, utils::constants::PG_TABLE_NAME};

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
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let password = compute_password_hash(user.password.as_ref().to_owned())
            .await
            .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        let sql = format!("insert into {} (email, password_hash, requires_2fa) values ($1, $2, $3)", PG_TABLE_NAME);
        sqlx::query(&sql)
            .bind(user.email.as_ref().expose_secret())
            .bind(password.expose_secret())
            .bind(user.requires_2fa)
            .execute(&self.pool)
            .await
            .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        Ok(())
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)] 
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let sql = format!("select * from {} where email = $1", PG_TABLE_NAME);
        let query = sqlx::query_as::<_, Users>(&sql);
        query
            .bind(email.as_ref().expose_secret())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| UserStoreError::UnexpectedError(e.into()))?
            .map(|u| {
                let email = Email::parse(Secret::new(u.email))
                    .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?;

                let password = Password::parse(Secret::new(u.password_hash))
                    .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?;

                Ok(User::new(email, password, u.requires_2fa))
            })
            .ok_or(UserStoreError::UserNotFound)?
    }
    
    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
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

#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: Secret<String>, 
    password_candidate: Secret<String>, 
) -> Result<()> {
    let res = tokio::task::spawn_blocking(move || {
        let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_password_hash.expose_secret())?;

        Argon2::default()
            .verify_password(password_candidate.expose_secret().as_bytes(), &expected_password_hash)
            .wrap_err("failed to verify password hash")
    })
    .await;

    res?
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>> {
    let current_span: tracing::Span = tracing::Span::current();

    let res = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.expose_secret().as_bytes(), &salt)?
            .to_string();
    
            Ok(Secret::new(password_hash))
            // Err(Box::new(std::io::Error::other("oh no")) as Box<dyn Error + Send +  Sync>)
            // Err(eyre!("oh no")) // testing
        })
    })
    .await;

    res?
}