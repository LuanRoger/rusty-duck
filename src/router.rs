use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::handlers::{ok, query};

pub fn router() -> Router {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "my_app=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Router::new()
        .route("/", get(query))
        .route("/system", get(ok))
        .layer(TraceLayer::new_for_http())
}
