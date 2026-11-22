pub mod assets;
pub mod bang;
mod handlers;
pub mod resolver;
mod router;

use tower_service::Service;
use worker::*;

use crate::router::ROUTER;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(ROUTER.clone().call(req).await?)
}
