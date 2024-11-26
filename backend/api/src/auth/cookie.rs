use anyhow::anyhow;
use axum_extra::extract::{
    cookie::{Cookie, Expiration},
    CookieJar,
};
use models::errors::AppResult;
use time::OffsetDateTime;

pub fn create_cookie<'a>(jwt_token: String, expiration: i64) -> AppResult<Cookie<'a>> {
    let cookie = Cookie::build(("jwt", jwt_token.clone()))
        .same_site(axum_extra::extract::cookie::SameSite::None)
        .http_only(true)
        .path("/")
        .domain(".lufy.cz")
        .expires(Expiration::DateTime(
            OffsetDateTime::from_unix_timestamp(expiration)
                .map_err(|_| anyhow!("Failed to create expiration time"))?,
        ))
        .secure(true)
        .build();

    Ok(cookie)
}
