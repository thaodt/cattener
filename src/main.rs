use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    env_logger::init();
    info!("Server started.");

    // Start database
    let db: sled::Db = sled::open("cattener_db")?;
    info!("Database started.");

    // register routes
    let app = cattener::create_app(db);

    // Start server on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
