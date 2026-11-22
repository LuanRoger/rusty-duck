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
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let path = req.uri().path();
    if path == "/system" || (path == "/" && req.uri().query().is_some_and(|q| !q.is_empty())) {
        return Ok(ROUTER.clone().call(req).await?);
    }

    let response = env.assets("ASSETS")?.fetch_request(req).await?;
    Ok(response.map(axum::body::Body::new))
}
