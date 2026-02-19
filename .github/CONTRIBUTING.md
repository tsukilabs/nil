# Contributing

## Requirements

- [Rust 1.95 (nightly)](https://www.rust-lang.org/tools/install)
- [Node 24](https://nodejs.org/en/download/current)
- [pnpm](https://pnpm.io/installation)
- [Diesel](https://diesel.rs/guides/getting-started#installing-diesel-cli)

To install Diesel, run:

```pwsh
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
```

> [!NOTE]
> The truth is you don’t really need to install Diesel unless you want to work with the remote server database.

If you also want to work on the Android version, you’ll need to go through some additional steps. The [official Tauri guide](https://v2.tauri.app/start/prerequisites/#android) should help you with that.

## Creating a server endpoint

When you add a new endpoint, there are a few places you need to update. Right now this is entirely manual, but we might be able to automate at least part of it in the future ~~(I hope)~~. Use what already exists as a reference for how the new one should look.

Here’s a simplified list of the steps you’ll need to take. In some cases, you can skip a few of them, for example, most of them won't need a response struct at all.

- [`nil-payload`](/crates/nil-payload)
  - [Create request and response structs](/crates/nil-payload/src/lib.rs)

- [`nil-server`](/crates/nil-server)
  - [Create a handler](/crates/nil-server/src/router/mod.rs)
  - [Register it as a new route](/crates/nil-server/src/router/mod.rs)

- [`nil-client`](/crates/nil-client)
  - [Update the client](/crates/nil-client/src/client/mod.rs)

- [`nil`](/app)
  - [Create a new command (Rust)](/app/src-tauri/src/command/mod.rs)
  - [Expose it to the frontend](/app/src-tauri/src/lib.rs)
  - [Create a new command (TypeScript)](/app/src/commands/index.ts)
  - [Declare an interface for the request struct](/app/src/lib/request)
  - [Declare an interface for the response struct](/app/src/lib/response)

- [`nil-lua`](/crates/nil-lua)
  - [Expose it to Lua](/crates/nil-lua/src/client/mod.rs)
  - [Declare Lua types](/crates/nil-lua/types)
    - [Client](/crates/nil-lua/types/script/client/index.d.lua)
    - [Request](/crates/nil-lua/types/request)
    - [Response](/crates/nil-lua/types/response)

## Environment variables

- `NIL_DATABASE_URL`
- `NIL_JWT_SECRET`
- `NIL_LOG_DIR`
- `NIL_LOG_TOWER_HTTP`
- `NIL_MINIFY_SOURCE`
- `NIL_REMOTE_SERVER_ADDR`
