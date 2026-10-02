use std::{
    collections::HashMap,
    sync::{LazyLock, OnceLock},
};

use fst::Map;
use serde::{Deserialize, Serialize};

use crate::assets::{BANGS_FST_FILE, BANGS_JSON_FILE, URLS_BIN_FILE};

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
static BANGS_FST: LazyLock<Map<&'static [u8]>> =
    LazyLock::new(|| Map::new(BANGS_FST_FILE).expect("Was not possible to create FST map"));

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

#[deprecated]
pub fn get_bangs() -> &'static HashMap<String, Bang> {
    BANGS.get_or_init(|| {
        serde_json::from_str::<Vec<Bang>>(BANGS_JSON_FILE)
            .unwrap_or_default()
            .into_iter()
            .map(|b| (b.trigger().to_string(), b))
            .collect()
    })
}

pub fn get_bangs_fst(trigger: &str) -> Option<&'static str> {
    let packed = BANGS_FST.get(trigger)?;

    let offset = (packed >> 32) as usize;
    let length = (packed & 0xFFFF) as usize;
    let url_bytes = URLS_BIN_FILE.get(offset..offset + length)?;

    str::from_utf8(url_bytes).ok()
}
