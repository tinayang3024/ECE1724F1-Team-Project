mod db;
mod server;
use anyhow::{Context, Result};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = PgPool::connect(&db::PG_CONNECTION_STR)
        .await
        .context("Failed to connect to the database")?;

    // start the server
    if let Err(e) = server::run_server(pool.clone()).await {
        eprintln!("Error while running server: {}", e);
        return Err(anyhow::anyhow!("Server failed to start").into());
    }

    Ok(())
}
