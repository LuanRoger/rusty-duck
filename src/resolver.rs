use crate::bang::{Bang, DEFAULT_BANG, DEFAULT_BANG_SYMBOL, QUERY_PLACEHOLDER, get_bangs};

pub struct Resolver {
    query: String,
    bang: Option<Bang>,
}

pub enum URLResult {
    Redirect(String),
    NotFound,
}

impl Resolver {
    fn new(query: String, bang: Option<Bang>) -> Self {
        Resolver { query, bang }
    }
}

pub fn parse(query: String) -> Resolver {
    let bang = extract_bang(&query);
    let bang = match bang {
        Some(bang) => get_bangs().get(bang),
        None => Some(*DEFAULT_BANG),
    };
    let query = parse_as_query(query);

    Resolver::new(query, bang.cloned())
}

pub fn mount_url(resolver: Resolver) -> URLResult {
    match resolver.bang {
        Some(bang) => URLResult::Redirect(bang.url().replace(QUERY_PLACEHOLDER, &resolver.query)),
        None => URLResult::NotFound,
    }
}

fn extract_bang(query: &str) -> Option<&str> {
    query
        .strip_prefix(DEFAULT_BANG_SYMBOL)
        .map(|query_rest| query_rest.split_whitespace())
        .and_then(|mut splited_query| splited_query.next())
}

fn parse_as_query(query: String) -> String {
    let has_bang = query.starts_with(DEFAULT_BANG_SYMBOL);
    if !has_bang {
        return query;
    }

    query
        .split_whitespace()
        .nth(1)
        .map(String::from)
        .expect("Query is not pressent")
}
