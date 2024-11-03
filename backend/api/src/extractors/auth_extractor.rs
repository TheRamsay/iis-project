use core::time;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Json};
use axum_extra::headers::{Authorization, Cookie, HeaderMapExt};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, Validation};
use models::errors::{AppError, AppResult};
use sea_orm::prelude::TimeDateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Default session duration is 7 days
const DEFAULT_SESSION_DURATION: time::Duration = time::Duration::from_secs(60 * 60 * 24 * 7);

pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
}

pub struct OptionalAuthUser(pub Option<AuthUser>);

#[derive(Debug, Deserialize, Serialize)]
struct AuthUserClaims {
    pub id: Uuid,
    pub username: String,
    pub exp: usize,
}

impl AuthUser {
    pub fn to_jwt(&self) -> String {
        let exp = Utc::now().timestamp() as usize + DEFAULT_SESSION_DURATION.as_secs() as usize;

        let claims = AuthUserClaims {
            id: self.id,
            username: self.username.clone(),
            exp,
        };

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
        )
        .expect("Error generating JWT token")
    }

    fn from_jwt(token: &str) -> AppResult<Self> {
        let token_data = jsonwebtoken::decode::<AuthUserClaims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Couldn't decode the JWT token".into()))?;

        Ok(AuthUser {
            id: token_data.claims.id,
            username: token_data.claims.username,
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
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(cookie) = parts.headers.typed_get::<Cookie>() {
            if let Some(token) = cookie.get("jwt") {
                let auth_user = AuthUser::from_jwt(token)?;
                return Ok(Self(Some(auth_user)));
            }
        }

        Ok(Self(None))
    }
}
