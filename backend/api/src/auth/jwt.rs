use redis::{self, Commands};

pub fn blacklist_token(
    redis_client: &redis::Client,
    jti: &str,
    exp: usize,
) -> redis::RedisResult<()> {
    println!("Blacklisting token with jti: {}", jti);
    let mut conn = redis_client.get_connection()?;
    let ttl = exp as isize - chrono::Utc::now().timestamp() as isize;
    conn.set_ex(jti, "blacklisted", ttl as u64)
}

pub fn is_token_blacklisted(redis_client: &redis::Client, jti: &str) -> redis::RedisResult<bool> {
    println!("Checking if token is blacklisted with jti: {}", jti);
    let mut conn = redis_client.get_connection()?;
    let exists: bool = conn.exists(jti)?;
    Ok(exists)
}
