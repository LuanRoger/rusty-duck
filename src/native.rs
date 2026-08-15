use anyhow::Result;

use rusty_duck::router::router;
use tokio::net::TcpListener;

const ADDRESS: &str = "0.0.0.0:3000";

pub async fn init() -> Result<()> {
    let app = router();

    let listener = TcpListener::bind(ADDRESS).await?;
    println!("Server listening on {}", ADDRESS);
    axum::serve(listener, app).await?;

    Ok(())
}
