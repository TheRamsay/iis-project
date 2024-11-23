use redis::{self, Commands};

pub fn blacklist_token(
    redis_client: &redis::Client,
    jti: &str,
    exp: usize,
) -> redis::RedisResult<()> {
    let mut conn = redis_client.get_connection()?;
    let ttl = exp as isize - chrono::Utc::now().timestamp() as isize;
    conn.set_ex(jti, "blacklisted", ttl as u64)
}

pub fn is_token_blacklisted(redis_client: &redis::Client, jti: &str) -> redis::RedisResult<bool> {
    let mut conn = redis_client.get_connection()?;
    let exists: bool = conn.exists(jti)?;
    Ok(exists)
}
