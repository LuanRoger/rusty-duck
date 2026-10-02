use std::{
    collections::HashMap,
    sync::{LazyLock, OnceLock},
};

use serde::{Deserialize, Serialize};

use crate::assets::BANGS_JSON_FILE;

pub const DEFAULT_BANG_SYMBOL: char = '!';
pub const DEFAULT_BANG_TRIGGER: &str = "g";
pub const QUERY_PLACEHOLDER: &str = "{{{s}}}";
pub static DEFAULT_BANG_TRIGGER_SYMBOL: LazyLock<String> =
    LazyLock::new(|| format!("{}{}", DEFAULT_BANG_SYMBOL, DEFAULT_BANG_TRIGGER));
pub static DEFAULT_BANG: LazyLock<&Bang> = LazyLock::new(|| {
    get_bangs()
        .get(DEFAULT_BANG_TRIGGER)
        .expect("Default bang not found")
});
static BANGS: OnceLock<HashMap<String, Bang>> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bang {
    s: String,
    t: String,
    u: String,
}

impl Bang {
    pub fn trigger(&self) -> &str {
        self.t.as_str()
    }

    pub fn url(&self) -> &str {
        self.u.as_str()
    }
}

pub fn get_bangs() -> &'static HashMap<String, Bang> {
    BANGS.get_or_init(|| {
        serde_json::from_str::<Vec<Bang>>(BANGS_JSON_FILE)
            .unwrap_or_default()
            .into_iter()
            .map(|b| (b.trigger().to_string(), b))
            .collect()
    })
}
