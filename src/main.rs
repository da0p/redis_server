use clap::Parser;

use redis_server::server;

#[derive(Parser, Debug)]
struct Options {
    /// port to bind
    #[arg(default_value_t=6379)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), redis_server::Error> {

    let options = Options::parse();

    let redis_server = server::RedisServer::new(options.port).await?;

    redis_server.run().await?;

    Ok(())
}
