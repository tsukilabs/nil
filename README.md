# Call of Nil

> [!NOTE]
> Call of Nil is still under development and not yet ready to play.

## Installation

You can download the latest version from our [website](https://nil.dev.br/download) or [GitHub Releases](https://github.com/tsukilabs/nil/releases/latest).

## Building from source

Call of Nil requires a [nightly Rust toolchain](https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust) to compile. You must also set up additional dependencies, such as [Node.js](https://nodejs.org/en/about). A complete list of requirements can be found in the contributing guide.

```sh
git clone https://github.com/tsukilabs/nil.git
cd nil
cargo +nightly -Zscript scripts/build-client.rs
```
