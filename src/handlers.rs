use anyhow::Result;
use axum::{
    Json,
    body::Body,
    extract::Query,
    http::{Response, StatusCode},
    response::Redirect,
};
use serde::{Deserialize, Serialize};
use worker::console_log;

use crate::{
    assets::FAVICON_FILE,
    bang::{DEFAULT_BANG_SYMBOL, DEFAULT_BANG_TRIGGER_SYMBOL, QUERY_PLACEHOLDER, get_bangs},
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
    let query = query.q.as_str();
    let mut query_partitions = query.split_whitespace();
    let bang_match = query_partitions
        .next()
        .and_then(|bang_operation_candidate| {
            let is_bang = bang_operation_candidate.starts_with(DEFAULT_BANG_SYMBOL);
            if !is_bang {
                return None;
            }

            bang_operation_candidate
                .rsplit_once(DEFAULT_BANG_SYMBOL)
                .map(|bang_operation| bang_operation.1)
        });
    let query = if bang_match.is_none() {
        query
    } else {
        query_partitions.next().expect("Query was not identified")
    };
    let bang_match = bang_match
        .unwrap_or(DEFAULT_BANG_TRIGGER_SYMBOL.as_str())
        .trim_start_matches(DEFAULT_BANG_SYMBOL);

    console_log!("Bang match: {}; Query: {}", bang_match, query);

    let bangs = get_bangs().await;
    if let Ok(bangs) = bangs {
        let bang = bangs.get(bang_match);
        match bang {
            Some(bang) => {
                let rediret_to = bang.url();
                let redirect_to = rediret_to.replace(QUERY_PLACEHOLDER, query);

                return Ok(Redirect::permanent(redirect_to.as_str()));
            }
            None => return Err(StatusCode::NOT_FOUND),
        }
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)
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
