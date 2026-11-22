pub mod assets;
pub mod bang;
mod handlers;
pub mod resolver;
mod router;

use worker::{Context, Env, Request, Response, Result, event};

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    router::route(req, env).await
}
