# rusty-duck

A lightweight [DuckDuckGo-style bang](https://duckduckgo.com/bangs) redirect service written in Rust and deployed as a Cloudflare Worker.

`rusty-duck` accepts a search query, resolves its leading bang against an embedded bang database, and redirects the request to the matching search provider. Queries without an explicit bang use Google (`!g`) by default.

## Technology

- [Rust](https://www.rust-lang.org/)
- [workers-rs](https://github.com/cloudflare/workers-rs) for native Cloudflare Workers request and response handling
- [Wrangler](https://developers.cloudflare.com/workers/wrangler/) for local development and deployment

## Try it out!

Access https://search.luanroger.dev/search?q=!yt+rust+cloudflare+workers.

The query is resolved against an embedded bang data set and redirected to the matching search provider, check the [`bangs.json`](https://raw.githubusercontent.com/LuanRoger/rusty-duck/refs/heads/main/public/bangs.json).

To use in your browser, register rusty-duck as a search engine in your browser's settings.

```
https://search.luanroger.dev/?q=%s
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with the `wasm32-unknown-unknown` target
- [Node.js](https://nodejs.org/) 26 (the version used in CI)
- [pnpm](https://pnpm.io/) 12.21 or later in the 12.x series
- [`worker-build`](https://crates.io/crates/worker-build) 0.9.0
- A Cloudflare account for deployment

Install the Rust WebAssembly target and Worker build tool:

```sh
rustup target add wasm32-unknown-unknown
cargo install worker-build --version 0.9.0 --locked
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

Turbo builds the Worker and Astro separately before starting Wrangler. Wrangler serves `apps/web/dist` directly as Worker Static Assets. Use the Astro dev server for frontend hot reload; to update the integrated Wrangler preview, rebuild the changed workspace from another terminal with `pnpm --filter web build` or `pnpm --filter app build`.

Cargo builds only the Rust code and regenerates its embedded bang database when `apps/app/public/bangs.json` changes. It does not run Astro or require Node.js or pnpm.

Wrangler serves the Worker at `http://localhost:8787` by default. Open `/` for the home page, or test the health endpoint and a redirect with:

```sh
curl http://localhost:8787/system
curl -i 'http://localhost:8787/search?q=!g+rust+language'
```

Application messages written with `worker::console_log!` appear in the Wrangler terminal.

## CI and deployment

GitHub Actions runs `pnpm build` (Turbo) and a Wrangler dry run on pull requests and pushes to `main`. After those checks pass on `main`, Wrangler deploys the Worker and Astro assets together. There is no separate web deployment.

Configure `CLOUDFLARE_API_TOKEN` as a GitHub Actions secret and `CLOUDFLARE_ACCOUNT_ID` as a GitHub Actions repository variable. Scope the token to the target Cloudflare account with **Workers Scripts: Edit** permission.

To validate the same bundle locally without uploading it:

```sh
pnpm build
pnpm --filter app run deploy --dry-run
```

## Observability

Cloudflare observability, persisted logs, and traces are disabled by default in `wrangler.toml`. To retain production logs in Cloudflare, enable the relevant `[observability]` and `[observability.logs]` options before deploying. Be mindful that search queries may contain sensitive information and are currently included in the request handler's console output.

## AI Policy

**WRITE**: AI has read-write permissions

<img src="https://0rwx.com/static/AI-W-dark.png" width="64" />

[See more](https://0rwx.com)
