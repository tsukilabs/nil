set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

alias app := build-app
alias clear := clean
alias ffi := build-ffi
alias ffi-pkg := build-ffi-package
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

fmt:
  @dprint fmt
  @cargo fmt --all
  @just --fmt --indentation "  " --quiet

lint *ARGS:
  @pnpm run lint {{ ARGS }}

type-check:
  @pnpm run -r --bail type-check

udeps:
  @cargo udeps --workspace

[group("docs")]
docs:
  @pnpm run -F docs dev

[group("docs")]
docs-js:
  @just vitepress
  @just typedoc

[group("docs")]
docs-rust:
  @cargo doc -p nil-* --open --no-deps --lib

[group("docs")]
typedoc:
  @pnpm run --recursive --if-present typedoc

[group("docs")]
vitepress:
  @pnpm run -F docs build

[group("ffi")]
build-ffi-package:
  @pnpm run -F @tsukilabs/nil-ffi build

[group("rsx")]
[private]
rsx FILE *ARGS:
  @cargo -Zscript scripts/{{ FILE }}.rs {{ ARGS }}

[group("rsx")]
build-app *ARGS:
  @just rsx build-app {{ ARGS }}

[group("ffi")]
[group("rsx")]
build-ffi *ARGS:
  @just rsx build-ffi {{ ARGS }}

[group("rsx")]
build-server *ARGS:
  @just rsx build-server {{ ARGS }}

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
  @just rsx build-app --preview

[group("rsx")]
server:
  @just rsx start-server

[group("rsx")]
sort-namegen:
  @just rsx sort-namegen

[group("rsx")]
test *ARGS:
  @just rsx test {{ ARGS }}
