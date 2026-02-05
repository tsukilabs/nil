# Contributing

## Requirements

- [Rust 1.95 (nightly)](https://www.rust-lang.org/tools/install)
- [Node 24](https://nodejs.org/en/download/current)
- [pnpm](https://pnpm.io/installation)
- [wasm-pack](https://drager.github.io/wasm-pack/installer/)
- [Diesel](https://diesel.rs/guides/getting-started#installing-diesel-cli)

To install Diesel, run:

```pwsh
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

If you also want to work on the Android version, you’ll need to go through some additional steps. The [official Tauri guide](https://v2.tauri.app/start/prerequisites/#android) should help you with that.

## Environment variables

- `NIL_DATABASE_URL`
- `NIL_JWT_SECRET`
- `NIL_LOG_DIR`
- `NIL_LOG_TOWER_HTTP`
- `NIL_MAP_WORKSPACE_DEPS`
- `NIL_MINIFY_SOURCE`
- `NIL_REMOTE_SERVER_ADDR`
