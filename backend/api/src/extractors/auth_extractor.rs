use core::time;

use anyhow::anyhow;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::headers::{Cookie, HeaderMapExt};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, Validation};
use models::{
    domain::user::UserType,
    errors::{AppError, AppResult},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::jwt::is_token_blacklisted, AppState};

// Default session duration is 1 day
const DEFAULT_SESSION_DURATION: time::Duration = time::Duration::from_secs(60 * 60 * 24);

pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub role: UserType,
    pub exp: usize,
}

pub struct OptionalAuthUser(pub Option<AuthUser>);

#[derive(Debug, Deserialize, Serialize)]
struct AuthUserClaims {
    pub id: Uuid,
    pub username: String,
    pub role: UserType,
    pub exp: usize,
}

impl AuthUser {
    pub fn new(id: Uuid, username: String, role: UserType) -> Self {
        let exp = Utc::now().timestamp() as usize + DEFAULT_SESSION_DURATION.as_secs() as usize;

        Self {
            id,
            username,
            role,
            exp,
        }
    }

    pub fn to_jwt(&self) -> String {
        let exp = Utc::now().timestamp() as usize + DEFAULT_SESSION_DURATION.as_secs() as usize;

        let claims = AuthUserClaims {
            id: self.id,
            username: self.username.clone(),
            role: self.role.clone(),
            exp,
        };

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
        )
        .expect("Error generating JWT token")
    }

    pub fn from_jwt(token: &str) -> AppResult<Self> {
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.reject_tokens_expiring_in_less_than =
            time::Duration::from_secs(5).as_secs() as u64;

        let token_data = jsonwebtoken::decode::<AuthUserClaims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation,
        )
        .map_err(|_| AppError::Unauthorized("Couldn't decode the JWT token".into()))?;

        Ok(AuthUser {
            id: token_data.claims.id,
            username: token_data.claims.username,
            role: token_data.claims.role,
            exp: token_data.claims.exp,
        })
    }
}

impl OptionalAuthUser {
    pub fn user_id(&self) -> Option<Uuid> {
        self.0.as_ref().map(|user| user.id)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match OptionalAuthUser::from_request_parts(parts, state).await? {
            OptionalAuthUser(Some(user)) => Ok(user),
            OptionalAuthUser(None) => Err(AppError::Unauthorized("Unauthorized".to_string())),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        if let Some(cookie) = parts.headers.typed_get::<Cookie>() {
            if let Some(token) = cookie.get("jwt") {
                if is_token_blacklisted(&state.redis_client, token)
                    .map_err(|e| AppError::Anyhow(anyhow!(e)))?
                {
                    return Err(AppError::Unauthorized("Token is blacklisted".into()));
                }

                let auth_user = AuthUser::from_jwt(token)?;
                return Ok(Self(Some(auth_user)));
            }
        }

        Ok(Self(None))
    }
}
