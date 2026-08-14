mod bang;
mod handlers;

use anyhow::Result;
use axum::{Router, routing::get};
use handlers::{ok, query};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "my_app=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(query))
        .route("/system", get(ok))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(ADDRESS).await?;
    println!("Server listening on {}", ADDRESS);
    axum::serve(listener, app).await?;

    Ok(())
}
