use std::sync::LazyLock;

use fst::Map;

use crate::assets::{BANGS_FST_FILE, URLS_BIN_FILE};

pub const DEFAULT_BANG_SYMBOL: char = '!';
pub const DEFAULT_BANG_TRIGGER: &str = "g";
pub const QUERY_PLACEHOLDER: &str = "{{{s}}}";
pub static DEFAULT_BANG_TRIGGER_SYMBOL: LazyLock<String> =
    LazyLock::new(|| format!("{}{}", DEFAULT_BANG_SYMBOL, DEFAULT_BANG_TRIGGER));
static BANGS_FST: LazyLock<Map<&'static [u8]>> =
    LazyLock::new(|| Map::new(BANGS_FST_FILE).expect("Was not possible to create FST map"));

pub fn get_bangs_fst(trigger: &str) -> Option<&'static str> {
    let packed = BANGS_FST.get(trigger)?;

    let offset = (packed >> 32) as usize;
    let length = (packed & 0xFFFF) as usize;
    let url_bytes = URLS_BIN_FILE.get(offset..offset + length)?;

    str::from_utf8(url_bytes).ok()
}
