fn main() {
    divan::main();
}

#[divan::bench_group()]
mod bang_find {
    use std::{collections::HashMap, hint::black_box, sync::OnceLock};

    use divan::Bencher;
    use rusty_duck::bang::get_bangs_fst;
    use serde::Deserialize;

    const BANGS_JSON_FILE: &str = include_str!("../public/bangs.json");
    static BANGS: OnceLock<HashMap<String, Bang>> = OnceLock::new();

    #[derive(Deserialize)]
    struct Bang {
        t: String,
        u: String,
    }

    impl Bang {
        fn trigger(&self) -> &str {
            self.t.as_str()
        }

        fn url(&self) -> &str {
            self.u.as_str()
        }
    }

    fn get_bangs() -> &'static HashMap<String, Bang> {
        BANGS.get_or_init(|| {
            serde_json::from_str::<Vec<Bang>>(BANGS_JSON_FILE)
                .unwrap_or_default()
                .into_iter()
                .map(|bang| (bang.trigger().to_string(), bang))
                .collect()
        })
    }

    #[divan::bench(args = ["g", "yt", "t3", "gov", "tc", "note", "medium", "adr", "pcworldbg", "zdnet", "sapblogs", "r", "li", "gh", "ste"])]
    fn list(bencher: Bencher, bang_to_find: &str) {
        let bangs = serde_json::from_str::<Vec<Bang>>(BANGS_JSON_FILE).unwrap_or_default();
        assert!(bangs.iter().any(|bang| bang.trigger() == bang_to_find));

        bencher.bench(|| {
            black_box(
                bangs
                    .iter()
                    .find(|bang| bang.trigger() == black_box(bang_to_find)),
            )
        });
    }

    #[divan::bench(args = ["g", "yt", "t3", "gov", "tc", "note", "medium", "adr", "pcworldbg", "zdnet", "sapblogs", "r", "li", "gh", "ste"])]
    fn hash(bencher: Bencher, bang_to_find: &str) {
        let bangs = get_bangs();
        assert!(bangs.contains_key(bang_to_find));

        bencher.bench(|| black_box(bangs.get(black_box(bang_to_find))));
    }

    #[divan::bench(args = ["g", "yt", "t3", "gov", "tc", "note", "medium", "adr", "pcworldbg", "zdnet", "sapblogs", "r", "li", "gh", "ste"])]
    fn fst(bencher: Bencher, bang_to_find: &str) {
        let expected_url = get_bangs()
            .get(bang_to_find)
            .expect("benchmark trigger should exist")
            .url();
        assert_eq!(get_bangs_fst(bang_to_find), Some(expected_url));

        bencher.bench(|| black_box(get_bangs_fst(black_box(bang_to_find))));
    }
}
