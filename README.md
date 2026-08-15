# rusty-duck

This project supports both a native Axum server and a Cloudflare Worker.

## Native server

```sh
cargo run --no-default-features --features native
```

The server listens on `0.0.0.0:3000`.

## Cloudflare Worker

Install Wrangler and build/deploy with:

```sh
wrangler deploy
```

`wrangler.toml` enables the `wasm` Cargo feature so the Worker entry point is compiled.

To build the Worker directly:

```sh
worker-build --release --features wasm
```
