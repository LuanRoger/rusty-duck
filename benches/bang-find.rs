fn main() {
    divan::main();
}

#[allow(deprecated)]
#[divan::bench_group()]
mod bang_find {
    use std::hint::black_box;

    use divan::Bencher;
    use rusty_duck::{
        assets::BANGS_JSON_FILE,
        bang::{Bang, get_bangs, get_bangs_fst},
    };

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
