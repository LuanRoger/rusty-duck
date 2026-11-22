use std::{
    env,
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
    process::Command,
};

use fst::MapBuilder;
use serde::Deserialize;

const WEB_PATHS_WATCH: &[&str] = &[
    "../web/src",
    "../web/public",
    "../web/astro.config.mjs",
    "../web/package.json",
    "../web/tsconfig.json",
    "../../pnpm-lock.yaml",
];
const WEB_PATH: &str = "../web";
const WEB_DIST_PATH: &str = "../web/dist";
const WEB_PUBLIC_PATH: &str = "public/web";

const BANGS_JSON_FILE: &str = "public/bangs.json";
const BANGS_FST_FILE: &str = "bangs.fst";
const URLS_BIN_FILE: &str = "urls.txt";

#[derive(Deserialize)]
pub struct Bang {
    t: String,
    u: String,
}

fn copy_dir(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn main() {
    for path in WEB_PATHS_WATCH {
        println!("cargo:rerun-if-changed={path}");
    }

    let status = Command::new("pnpm")
        .args(["--dir", WEB_PATH, "build"])
        .status()
        .expect("Failed to start Astro build; ensure pnpm is installed");
    assert!(status.success(), "Astro build failed: {status}");

    let site = Path::new(WEB_PUBLIC_PATH);
    if site.exists() {
        fs::remove_dir_all(site).expect("Failed to remove previous Astro build");
    }
    copy_dir(Path::new(WEB_DIST_PATH), site).expect("Failed to copy Astro build");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR needs to be available"));
    let bangs_json_raw = fs::read_to_string(BANGS_JSON_FILE).expect("Bang file was not found");
    let mut bangs: Vec<Bang> =
        serde_json::from_str(&bangs_json_raw).expect("Was not possible to parse JSON bang file");

    bangs.sort_unstable_by(|x, y| x.t.cmp(&y.t));

    let bangs_fst_out_dir = out_dir.join(BANGS_FST_FILE);
    let urls_bin_out_dir = out_dir.join(URLS_BIN_FILE);
    let bangs_file = File::create(bangs_fst_out_dir).expect("Was not possible to create the file");
    let bangs_file_writer = BufWriter::new(bangs_file);

    let mut builder =
        MapBuilder::new(bangs_file_writer).expect("Was not possible to create the FST builder");
    let mut urls = Vec::new();

    for bang in bangs {
        let offset = urls.len();
        let bytes = bang.u.as_bytes();
        let lenght = bytes.len();

        assert!(lenght <= u16::MAX as usize);

        urls.extend_from_slice(bytes);

        let packed = (offset as u64) << 16 | lenght as u64;

        builder
            .insert(&bang.t, packed)
            .expect("Was not possible to insert into builder");
    }

    builder
        .finish()
        .expect("Was not possible to finish the builder");
    fs::write(urls_bin_out_dir, urls).expect("Was not possible to write URLs bin file");
}
