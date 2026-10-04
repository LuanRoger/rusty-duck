pub const FAVICON_FILE: &[u8] = include_bytes!("../public/favicon.ico");

pub const BANGS_FST_FILE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bangs.fst"));
pub const URLS_FILE: &str = include_str!(concat!(env!("OUT_DIR"), "/urls.txt"));
