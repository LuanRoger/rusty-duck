use worker::{Response, Result};

use crate::resolver::{URLResult, mount_url, parse};

pub fn query(raw_query: &str) -> Result<Response> {
    let mut queries = form_urlencoded::parse(raw_query.as_bytes())
        .filter_map(|(key, value)| (key == "q").then_some(value));
    
    let Some(query) = queries.next() else {
        return bad_query("Failed to deserialize query string: missing field `q`");
    };
    if queries.next().is_some() {
        return bad_query("Failed to deserialize query string: duplicate field `q`");
    }

    match mount_url(parse(&query)) {
        URLResult::Redirect(url) => Ok(Response::builder()
            .with_status(307)
            .with_header("Location", &url)?
            .empty()),
        URLResult::NotFound => Ok(Response::builder().with_status(404).empty()),
    }
}

pub fn health() -> Result<Response> {
    Ok(Response::builder()
        .with_header("Content-Type", "application/json")?
        .fixed(br#"{"message":"OK"}"#.to_vec()))
}

fn bad_query(message: &str) -> Result<Response> {
    Ok(Response::ok(message)?.with_status(400))
}
