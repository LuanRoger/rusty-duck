use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::{fs, sync::OnceCell};

const BANGS_FILE: &str = "bangs.json";
pub const DEFAULT_BANG_SYMBOL: char = '!';
pub const DEFAULT_BANG_TRIGGER: &str = "g";
pub const QUERY_PLACEHOLDER: &str = "{{{s}}}";
pub static DEFAULT_BANG_TRIGGER_SYMBOL: LazyLock<String> =
    LazyLock::new(|| format!("{}{}", DEFAULT_BANG_SYMBOL, DEFAULT_BANG_TRIGGER));
pub static BANG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"!(\S+)").unwrap());
pub static BANGS: OnceCell<Vec<Bang>> = OnceCell::const_new();

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

async fn read_bangs(file_path: &str) -> Result<Vec<Bang>> {
    let content = fs::read(file_path).await?;
    let result = serde_json::from_slice::<Vec<Bang>>(&content)?;

    Ok(result)
}

pub async fn get_bangs() -> Result<&'static Vec<Bang>> {
    let result = BANGS
        .get_or_init(async || {
            let bangs = read_bangs(BANGS_FILE).await.unwrap_or(vec![]);

            println!("{} bangs initialized", bangs.len());
            bangs
        })
        .await;

    Ok(result)
}
