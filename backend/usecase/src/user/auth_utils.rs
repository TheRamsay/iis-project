use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2,
};
use models::errors::{AppError, AppResult};
use tokio;

pub async fn verify_password(password: String, hash: String) -> AppResult<()> {
    tokio::task::spawn_blocking(move || {
        let password_hash = PasswordHash::new(&hash).map_err(|e| AppError::Anyhow(anyhow!(e)))?;

        password_hash
            .verify_password(&[&Argon2::default()], password.as_bytes())
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => {
                    AppError::Unauthorized("Invalid password".to_string())
                }
                _ => AppError::Anyhow(anyhow!(e)),
            })
    })
    .await
    .map_err(|e| AppError::Anyhow(anyhow!(e)))??;

    Ok(())
}

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Anyhow(anyhow!(e)))?
        .to_string())
}
