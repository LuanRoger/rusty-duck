# rusty-duck

A lightweight [DuckDuckGo-style bang](https://duckduckgo.com/bangs) redirect service written in Rust and deployed as a Cloudflare Worker.

`rusty-duck` accepts a search query, resolves its leading bang against an embedded bang database, and redirects the request to the matching search provider. Queries without an explicit bang use Google (`!g`) by default.

## Technology

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum) for routing and HTTP handlers
- [workers-rs](https://github.com/cloudflare/workers-rs) for the Cloudflare Workers runtime
- [Wrangler](https://developers.cloudflare.com/workers/wrangler/) for local development and deployment

## Try it out!

Access https://search.luanroger.dev?q=!yt+rust+cloudflare+workers.

The query is resolved against an embedded bang data set and redirected to the matching search provider, check the [`bangs.json`](https://raw.githubusercontent.com/LuanRoger/rusty-duck/refs/heads/main/public/bangs.json).

To use in your browser, register rusty-duck as a search engine in your browser's settings.

```
https://search.luanroger.dev/?q=%s
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with the `wasm32-unknown-unknown` target
- [Node.js](https://nodejs.org/)
- [pnpm](https://pnpm.io/)
- A Cloudflare account for deployment

Install the Rust WebAssembly target if it is not already available:

```sh
rustup target add wasm32-unknown-unknown
```

## Local development

Install the JavaScript tooling:

```sh
pnpm install
```

Start the Worker locally with Wrangler:

```sh
pnpm dev
```

Wrangler serves the Worker at `http://localhost:8787` by default. Test the health endpoint and a redirect with:

```sh
curl http://localhost:8787/system
curl -i 'http://localhost:8787/?q=!g+rust+language'
```

Application messages written with `worker::console_log!` appear in the Wrangler terminal.

## Observability

Cloudflare observability, persisted logs, and traces are disabled by default in `wrangler.toml`. To retain production logs in Cloudflare, enable the relevant `[observability]` and `[observability.logs]` options before deploying. Be mindful that search queries may contain sensitive information and are currently included in the request handler's console output.
