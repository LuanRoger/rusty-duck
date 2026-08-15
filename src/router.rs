use axum::{Router, routing::get};

use crate::handlers::{ok, query};

pub fn router() -> Router {
    Router::new()
        .route("/", get(query))
        .route("/system", get(ok))
}
