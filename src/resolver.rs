use crate::bang::{DEFAULT_BANG_SYMBOL, DEFAULT_BANG_TRIGGER, QUERY_PLACEHOLDER, get_bangs_fst};

#[derive(Debug)]
pub struct Resolver {
    query: String,
    url: Option<&'static str>,
}

#[derive(Debug)]
pub enum URLResult {
    Redirect(String),
    NotFound,
}

impl Resolver {
    fn new(query: String, url: Option<&'static str>) -> Self {
        Resolver { query, url }
    }
}

pub fn parse(query: &str) -> Resolver {
    let bang = extract_bang(query).unwrap_or(DEFAULT_BANG_TRIGGER);
    let query = parse_as_query(query);
    let url = get_bangs_fst(bang);

    Resolver::new(query, url)
}

pub fn mount_url(resolver: Resolver) -> URLResult {
    match resolver.url {
        Some(url) => URLResult::Redirect(url.replace(QUERY_PLACEHOLDER, &resolver.query)),
        None => URLResult::NotFound,
    }
}

fn extract_bang(query: &str) -> Option<&str> {
    query
        .strip_prefix(DEFAULT_BANG_SYMBOL)
        .map(|query_rest| query_rest.split_whitespace())
        .and_then(|mut splited_query| splited_query.next())
}

fn parse_as_query(query: &'_ str) -> String {
    let has_bang = query.starts_with(DEFAULT_BANG_SYMBOL);
    if !has_bang {
        return String::from(query);
    }

    query
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ")
}
