use anyhow::anyhow;
use argon2::{Argon2, PasswordHash};
use axum::{extract::State, routing::post};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use models::errors::{AppError, AppResult};
use repository::user_repository::{self, DbUserRepository, UserRepository};
use serde::{Deserialize, Serialize};
use usecase::user::{
    auth_utils::verify_password,
    register_user::{RegisterUserInput, RegisterUserUseCase},
};

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
pub fn auth_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register", post(register))
}
