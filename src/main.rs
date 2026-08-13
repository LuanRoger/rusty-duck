mod bang;

use anyhow::Result;
use axum::{Router, extract::Query, http::StatusCode, response::Redirect, routing::get};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::bang::{
    BANG_REGEX, BANGS, DEFAULT_BANG_SYMBOL, DEFAULT_BANG_TRIGGER_SYMBOL, QUERY_PLACEHOLDER,
};

#[derive(Debug, Serialize, Deserialize)]
struct GetQuery {
    q: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler(Query(query): Query<GetQuery>) -> Result<Redirect, StatusCode> {
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

    let bang = BANGS.iter().find(|b| b.trigger() == bang_match);
    match bang {
        Some(bang) => {
            let rediret_to = bang.url();
            let redirect_to = rediret_to.replace(QUERY_PLACEHOLDER, query);

            Ok(Redirect::permanent(redirect_to.as_str()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
