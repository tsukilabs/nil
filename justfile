set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

alias build := build-client
alias clear := clean
alias format := fmt

help:
  @just --list

init:
  @pnpm ci
  @rustup update
  @rustup toolchain install nightly
  @cargo install tauri-cli dprint miho
  @cargo install diesel_cli --no-default-features --features "sqlite-bundled"
  @just generate-bindings --force

docs:
  @pnpm run -F docs dev

docs-rust:
  @cargo doc -p nil-* --open --no-deps --lib --document-private-items

fmt:
  @dprint fmt
  @cargo fmt --all
  @just --fmt --indentation "  " --quiet

lint *ARGS:
  @pnpm run lint {{ ARGS }}

type-check:
  @pnpm run -r --bail type-check

udeps:
  @cargo udeps

[private]
rsx FILE *ARGS:
  @cargo -Zscript scripts/{{ FILE }}.rs {{ ARGS }}

[group("rsx")]
build-client *ARGS:
  @just rsx build-client {{ ARGS }}

[group("rsx")]
bump *ARGS:
  @just rsx bump {{ ARGS }}

[group("rsx")]
clippy *ARGS:
  @just rsx run-clippy {{ ARGS }}

[group("rsx")]
clean *ARGS:
  @just rsx clean {{ ARGS }}
  @cargo clean

[group("rsx")]
dev *ARGS:
  @just rsx start-dev {{ ARGS }}

[group("rsx")]
generate-bindings *ARGS:
  @just rsx generate-bindings {{ ARGS }}

[group("rsx")]
miri:
  @just rsx test --miri

[group("rsx")]
preview:
  @just rsx build-client --preview

[group("rsx")]
server:
  @just rsx start-server

[group("rsx")]
test *ARGS:
  @just rsx test {{ ARGS }}
