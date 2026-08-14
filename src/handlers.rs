use anyhow::Result;
use axum::{Json, extract::Query, http::StatusCode, response::Redirect};
use serde::{Deserialize, Serialize};

use crate::bang::{
    BANG_REGEX, DEFAULT_BANG_SYMBOL, DEFAULT_BANG_TRIGGER_SYMBOL, QUERY_PLACEHOLDER, get_bangs,
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
    let bang_match = {
        let regex_match = BANG_REGEX.find(query);
        regex_match.map(|m| m.as_str())
    }
    .unwrap_or(DEFAULT_BANG_TRIGGER_SYMBOL.as_str());
    let query = query.trim_start_matches(bang_match).trim();
    let bang_match = bang_match.trim_start_matches(DEFAULT_BANG_SYMBOL);

    dbg!(bang_match);
    dbg!(query);

    let bangs = get_bangs().await;
    if let Ok(bangs) = bangs {
        let bang = bangs.iter().find(|b| b.trigger() == bang_match);
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

pub async fn ok() -> Result<Json<HealthResponse>, StatusCode> {
    Ok(Json(HealthResponse::default()))
}
