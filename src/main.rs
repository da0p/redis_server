use redis_server::server;

use std::env;

#[tokio::main]
async fn main() -> Result<(), redis_server::Error> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6379".to_string());

    let redis_server = server::RedisServer::new(&addr).await?;

    redis_server.execute().await?;

    Ok(())
}
