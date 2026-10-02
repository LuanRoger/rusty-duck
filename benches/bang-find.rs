fn main() {
    divan::main();
}

#[divan::bench_group()]
mod bang_find {
    use divan::Bencher;
    use rusty_duck::{
        assets::BANGS_JSON_FILE,
        bang::{Bang, get_bangs},
    };

    #[divan::bench(args = ["g", "yt", "t3", "gov", "tc", "note", "medium", "adr", "pcworldbg", "zdnet", "sapblogs", "r", "li", "gh", "ste"])]
    fn list(bencher: Bencher, bang_to_find: &str) {
        let bangs = serde_json::from_str::<Vec<Bang>>(BANGS_JSON_FILE).unwrap_or(vec![]);

        bencher.bench(|| {
            let bang = bangs.iter().find(|b| b.trigger() == bang_to_find);
            assert!(bang.is_some());
            assert_eq!(bang.unwrap().trigger(), bang_to_find);
        });
    }

    #[divan::bench(args = ["g", "yt", "t3", "gov", "tc", "note", "medium", "adr", "pcworldbg", "zdnet", "sapblogs", "r", "li", "gh", "ste"])]
    async fn hash(bang_to_find: &str) {
        let bang = get_bangs().get(bang_to_find);
        assert!(bang.is_some());
        assert_eq!(bang.unwrap().trigger(), bang_to_find);
    }
}
