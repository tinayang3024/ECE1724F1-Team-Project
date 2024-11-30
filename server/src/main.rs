mod server;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = sqlx::PgPool::connect(db::PG_CONNECTION_STR).await?;
    server::run_server(pool.clone()).await;
    
    Ok(())
}
