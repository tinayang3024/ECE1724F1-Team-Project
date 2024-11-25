mod server;
// mod frontend;
// mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // db_pool: database pool connection

    // start frontend
    //frontend::run_frontend().await;
    
    server::run_server(db_pool.clone()).await;
    
    Ok(())
}
