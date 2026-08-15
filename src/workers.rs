use tower_service::Service;
use worker::*;

use crate::handlers::{ok, query};
use worker::axum::{Router, routing::get};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<http::Response<axum::body::Body>> {
    let app = Router::new()
        .route("/", get(query))
        .route("/system", get(ok));

    Ok(app.call(req).await?)
}
