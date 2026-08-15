mod bang;
mod handlers;
mod router;

use tower_service::Service;
use worker::*;

use crate::router::router;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}
