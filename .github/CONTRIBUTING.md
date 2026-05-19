# Contributing

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/current)
- [pnpm](https://pnpm.io/installation)
- [just](https://just.systems/man/en/installation.html)

After setting up all the requirements, run:

```pwsh
just init
```

If you also want to work on the Android version, you’ll need to go through some additional steps. The [official Tauri guide](https://v2.tauri.app/start/prerequisites/#android) should help you with that.

## Creating a server endpoint

When you add a new endpoint, there are a few places you need to update. Right now this is entirely manual, but we might be able to automate at least part of it in the future ~~(I hope)~~. Use what already exists as a reference for how the new one should look.

Here’s a simplified list of the steps you’ll need to take.

- [`nil-payload`](/crates/nil-payload)
  - [Create request and response structs](/crates/nil-payload/src/lib.rs)

- [`nil-server`](/crates/nil-server)
  - [Create a handler](/crates/nil-server/src/router/mod.rs)
  - [Register a new route](/crates/nil-server/src/router/mod.rs)

- [`nil-client`](/crates/nil-client)
  - [Update the client](/crates/nil-client/src/client/mod.rs)
  - [Implement the `Request` trait](/crates/nil-client/src/request.rs)

- [`nil`](/app)
  - [Create a new Rust command](/app/src-tauri/src/command/mod.rs)
  - [Expose it to TypeScript](/app/src-tauri/src/lib.rs)
  - [Create a new TypeScript command](/app/src/commands/index.ts)
