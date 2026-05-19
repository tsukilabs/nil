set windows-shell := ["powershell.exe", "-Command"]

alias build := build-client
alias clear := clean
alias format := fmt

help:
  @just --list

init:
  @pnpm ci
  @rustup update
  @rustup toolchain install nightly
  @cargo install tauri-cli dprint
  @cargo +nightly install miho
  @cargo install diesel_cli --no-default-features --features "sqlite-bundled"
  @just generate-bindings --force

docs:
  @cargo +nightly doc -p nil-* --open --no-deps --lib --document-private-items

fmt:
  @dprint fmt
  @cargo fmt --all
  @just --fmt --indentation "  " --quiet

lint *ARGS:
  @pnpm run lint {{ ARGS }}

type-check:
  @pnpm run -r --bail type-check

udeps:
  @cargo +nightly udeps

[group('script'), private]
rsx FILE *ARGS:
  @cargo +nightly -Zscript scripts/{{ FILE }}.rs {{ ARGS }}

[group('script')]
build-client *ARGS:
  @just rsx build-client {{ ARGS }}

[group('script')]
bump *ARGS:
  @just rsx bump {{ ARGS }}

[group('script')]
clippy *ARGS:
  @just rsx run-clippy {{ ARGS }}

[group('script')]
clean *ARGS:
  @just rsx clean {{ ARGS }}
  @cargo clean

[group('script')]
dev *ARGS:
  @just rsx start-dev {{ ARGS }}

[group('script')]
generate-bindings *ARGS:
  @just rsx generate-bindings {{ ARGS }}

[group('script')]
miri:
  @just rsx test --miri

[group('script')]
preview:
  @just rsx build-client --preview

[group('script')]
server:
  @just rsx start-server

[group('script')]
test *ARGS:
  @just rsx test {{ ARGS }}
