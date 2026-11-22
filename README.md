# rusty-duck

A lightweight [DuckDuckGo-style bang](https://duckduckgo.com/bangs) redirect service written in Rust and deployed as a Cloudflare Worker.

`rusty-duck` accepts a search query, resolves its leading bang against an embedded bang database, and redirects the request to the matching search provider. Queries without an explicit bang use Google (`!g`) by default.

## Technology

- [Rust](https://www.rust-lang.org/)
- [workers-rs](https://github.com/cloudflare/workers-rs) for native Cloudflare Workers request and response handling
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

The Worker build runs the Astro build and copies its output into `apps/app/public/web` for Wrangler to deploy as static assets. Install the workspace dependencies before building the Worker; Node.js and pnpm are required even when running Cargo directly.

Wrangler serves the Worker at `http://localhost:8787` by default. Open `/` for the home page, or test the health endpoint and a redirect with:

```sh
curl http://localhost:8787/system
curl -i 'http://localhost:8787/?q=!g+rust+language'
```

Application messages written with `worker::console_log!` appear in the Wrangler terminal.

## Observability

Cloudflare observability, persisted logs, and traces are disabled by default in `wrangler.toml`. To retain production logs in Cloudflare, enable the relevant `[observability]` and `[observability.logs]` options before deploying. Be mindful that search queries may contain sensitive information and are currently included in the request handler's console output.
