use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bang {
    c: &'static str,
    d: &'static str,
    r: u32,
    s: &'static str,
    sc: &'static str,
    t: &'static str,
    u: &'static str,
}

impl Bang {
    pub fn trigger(&self) -> &str {
        self.t
    }

    pub fn url(&self) -> &str {
        self.u
    }
}

pub const DEFAULT_BANG_SYMBOL: char = '!';
pub const DEFAULT_BANG_TRIGGER: &str = "g";
pub const QUERY_PLACEHOLDER: &str = "{{{s}}}";
pub static DEFAULT_BANG_TRIGGER_SYMBOL: LazyLock<String> =
    LazyLock::new(|| format!("{}{}", DEFAULT_BANG_SYMBOL, DEFAULT_BANG_TRIGGER));
pub static BANG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"!(\S+)").unwrap());

pub static BANGS: &[Bang] = &[
    Bang {
        c: "AI",
        d: "www.t3.chat",
        r: 0,
        s: "T3 Chat",
        sc: "AI",
        t: "t3",
        u: "https://www.t3.chat/new?q={{{s}}}",
    },
    Bang {
        c: "Tech",
        d: "cse.google.com",
        r: 0,
        s: "9to5google",
        sc: "Blogs",
        t: "95g",
        u: "https://cse.google.com/cse?cx=008464549922976904202:uxmexxzm3k4&q={{{s}}}",
    },
    Bang {
        c: "Multimedia",
        d: "www.youtube.com",
        r: 463021,
        s: "YouTube",
        sc: "Video",
        t: "yt",
        u: "https://www.youtube.com/results?search_query={{{s}}}",
    },
];
