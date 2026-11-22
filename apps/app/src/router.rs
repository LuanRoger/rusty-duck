use worker::{Env, Method, Request, Response, ResponseBody, Result};

use crate::handlers::{health, query};

pub async fn route(req: Request, env: Env) -> Result<Response> {
    match req.path().as_str() {
        "/system" => return get(req.method(), health),
        "/search" => {
            let url = req.inner().url();
            let url = url.split('#').next().unwrap_or_default();
            if let Some((_, raw_query)) = url.split_once('?')
                && !raw_query.is_empty()
            {
                return get(req.method(), || query(raw_query));
            }
        }
        _ => {}
    }

    env.assets("ASSETS")?.fetch_request(req).await
}

fn get(method: Method, handler: impl FnOnce() -> Result<Response>) -> Result<Response> {
    match method {
        Method::Get => handler(),
        Method::Head => {
            let (builder, body) = handler()?.into_parts();
            let length = match body {
                ResponseBody::Empty => 0,
                ResponseBody::Body(bytes) => bytes.len(),
                ResponseBody::Stream(_) => return Ok(builder.empty()),
            };
            Ok(builder
                .with_header("Content-Length", &length.to_string())?
                .empty())
        }
        _ => Ok(Response::builder()
            .with_status(405)
            .with_header("Allow", "GET,HEAD")?
            .empty()),
    }
}
