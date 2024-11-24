use anyhow::anyhow;
use axum::{extract::State, routing::post};
use axum_extra::extract::{
    cookie::{Cookie, Expiration},
    CookieJar,
};
use models::errors::{AppError, AppResult};
use repository::user_repository::UserRepository;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use usecase::user::{
    auth_utils::verify_password,
    register_user::{RegisterUserInput, RegisterUserUseCase},
};

use crate::{
    extractors::{auth_extractor::AuthUser, json_extractor::Json},
    AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid username".into()))?;

    if user.is_blocked {
        return Err(AppError::Unauthorized("User is blocked".into()));
    }

    verify_password(payload.password, user.password_hash).await?;

    let auth_user = AuthUser::new(user.id.into(), user.username.clone(), user.user_type);
    let token = auth_user.to_jwt(&state.jwt_secret);

    let cookie = Cookie::build(("jwt", token.clone()))
        .same_site(axum_extra::extract::cookie::SameSite::None)
        .http_only(true)
        .path("/")
        .expires(Expiration::DateTime(
            OffsetDateTime::from_unix_timestamp(auth_user.exp as i64)
                .map_err(|_| anyhow!("Failed to create expiration time"))?,
        ))
        .secure(true);

    let jar = CookieJar::new().add(cookie.clone());

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    avatar_url: Option<String>,
}

async fn register(
    state: State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<(CookieJar, ())> {
    let register_user_usecase =
        RegisterUserUseCase::new(state.user_repository.clone(), state.wall_repository.clone());

    let inserted = register_user_usecase
        .execute(RegisterUserInput {
            display_name: payload.username.clone(),
            username: payload.username.clone(),
            email: payload.email.clone(),
            avatar_url: payload.avatar_url.clone(),
            user_type: models::domain::user::UserType::Regular,
            password: payload.password.clone(),
        })
        .await?;

    let auth_user = AuthUser::new(
        inserted.id,
        payload.username,
        models::domain::user::UserType::Regular,
    );
    let token = auth_user.to_jwt(&state.jwt_secret);

    let cookie = Cookie::build(("jwt", token.clone()))
        .same_site(axum_extra::extract::cookie::SameSite::None)
        .http_only(true)
        .path("/")
        .expires(Expiration::DateTime(
            OffsetDateTime::from_unix_timestamp(auth_user.exp as i64)
                .map_err(|_| anyhow!("Failed to create expiration time"))?,
        ))
        .secure(true);

    let jar = CookieJar::new().add(cookie.clone());

    Ok((jar, ()))
}

pub fn auth_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/register", post(register))
}
