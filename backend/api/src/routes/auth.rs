use anyhow::anyhow;
use argon2::{Argon2, PasswordHash};
use axum::{extract::State, routing::post};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use models::errors::{AppError, AppResult};
use repository::user_repository::{self, DbUserRepository, UserRepository};
use serde::{Deserialize, Serialize};
use usecase::user::register_user::{RegisterUserInput, RegisterUserUseCase};

use crate::{
    extractors::{auth_extractor::AuthUser, json_extractor::Json},
    AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    username: String,
}

pub async fn login(
    state: State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<(CookieJar, Json<LoginResponse>)> {
    let user_repository = state.user_repository.clone();

    let user = user_repository
        .get_by_username(payload.username.clone())
        .await?;

    if user.is_none() {
        return Err(AppError::Unauthorized("Invalid username".to_string()));
    }

    let user = user.unwrap();

    verify_password(payload.password, user.password_hash).await?;

    let token = AuthUser {
        id: user.id.into(),
        username: user.username.clone(),
    }
    .to_jwt();

    let cookie = Cookie::build(("jwt", token))
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .http_only(true)
        .secure(true);

    let jar = CookieJar::new().add(cookie);

    Ok((
        jar,
        Json(LoginResponse {
            username: user.username.clone(),
        }),
    ))
}

async fn logout(jar: CookieJar) -> AppResult<()> {
    let _ = jar.remove(Cookie::from("jwt"));

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    avatar_url: Option<String>,
}

async fn register(state: State<AppState>, Json(payload): Json<RegisterRequest>) -> AppResult<()> {
    let register_user_usecase =
        RegisterUserUseCase::new(state.user_repository.clone(), state.wall_repository.clone());

    register_user_usecase
        .execute(RegisterUserInput {
            display_name: payload.username.clone(),
            username: payload.username.clone(),
            email: payload.email.clone(),
            avatar_url: payload.avatar_url.clone(),
            user_type: models::domain::user::UserType::Regular,
            password: payload.password.clone(),
        })
        .await?;

    Ok(())
}

async fn verify_password(password: String, hash: String) -> AppResult<()> {
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

pub fn auth_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register", post(register))
}
