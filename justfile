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

build-client *ARGS:
  @just rsx build-client {{ ARGS }}

bump *ARGS:
  @just rsx bump {{ ARGS }}

clippy *ARGS:
  @just rsx run-clippy {{ ARGS }}

clean *ARGS:
  @just rsx clean {{ ARGS }}
  @cargo clean

dev *ARGS:
  @just rsx start-dev {{ ARGS }}

generate-bindings *ARGS:
  @just rsx generate-bindings {{ ARGS }}

miri:
  @just rsx test --miri

preview:
  @just rsx build-client --preview

server:
  @just rsx start-server

test *ARGS:
  @just rsx test {{ ARGS }}
