use axum::{Router, routing::get};

use crate::handlers::{favicon, ok, query};

pub fn router() -> Router {
    Router::new()
        .route("/favicon.ico", get(favicon))
        .route("/", get(query))
        .route("/system", get(ok))
}
