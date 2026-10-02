use anyhow::Result;
use axum::{
    Json,
    body::Body,
    extract::Query,
    http::{Response, StatusCode},
    response::Redirect,
};
use serde::{Deserialize, Serialize};

use crate::{
    assets::FAVICON_FILE,
    resolver::{
        URLResult::{self},
        mount_url, parse,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct HandlerQuery {
    q: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    message: String,
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self {
            message: String::from("OK"),
        }
    }
}

pub async fn query(Query(query): Query<HandlerQuery>) -> Result<Redirect, StatusCode> {
    let parsed_query = parse(query.q);
    let resolved_url = mount_url(parsed_query);

    match resolved_url {
        URLResult::Redirect(url) => Ok(Redirect::temporary(&url)),
        URLResult::NotFound => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn favicon() -> Response<Body> {
    Response::builder()
        .header("Content-Type", "image/x-icon")
        .status(StatusCode::OK)
        .body(Body::from(FAVICON_FILE))
        .expect("favicon response has a valid static configuration")
}

pub async fn ok() -> Result<Json<HealthResponse>, StatusCode> {
    Ok(Json(HealthResponse::default()))
}
