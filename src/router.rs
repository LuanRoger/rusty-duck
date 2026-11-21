use std::sync::LazyLock;

use axum::{Router, routing::get};

use crate::handlers::{favicon, ok, query};

pub static ROUTER: LazyLock<Router> = LazyLock::new(|| {
    Router::new()
        .route("/favicon.ico", get(favicon))
        .route("/", get(query))
        .route("/system", get(ok))
});
