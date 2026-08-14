mod bang;
mod handlers;

use anyhow::Result;
use axum::{Router, routing::get};
use handlers::{query, ok};
use tokio::net::TcpListener;

const ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(query))
        .route("/system", get(ok));

    let listener = TcpListener::bind(ADDRESS).await?;
    println!("Server listening on {}", ADDRESS);
    axum::serve(listener, app).await?;

    Ok(())
}
