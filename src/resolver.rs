use urlencoding::encode;

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
    let (bang, query) = split_query(query);
    let url = get_bangs_fst(bang);

    Resolver::new(String::from(query), url)
}

pub fn mount_url(resolver: Resolver) -> URLResult {
    match resolver.url {
        Some(url) => {
            let Some((prefix, suffix)) = url.split_once(QUERY_PLACEHOLDER) else {
                return URLResult::NotFound;
            };
            let query = encode(&resolver.query);
            let mut result = String::with_capacity(prefix.len() + query.len() + suffix.len());

            result.push_str(prefix);
            result.push_str(&query);
            result.push_str(suffix);
            URLResult::Redirect(result)
        }
        None => URLResult::NotFound,
    }
}

fn split_query(query: &str) -> (&str, &str) {
    let Some(rest) = query.strip_prefix(DEFAULT_BANG_SYMBOL) else {
        return (DEFAULT_BANG_TRIGGER, query);
    };

    match rest.find(char::is_whitespace) {
        Some(index) => (&rest[..index], rest[index..].trim()),
        None => (rest, ""),
    }
}
