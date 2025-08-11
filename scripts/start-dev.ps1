# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('A')]
  [switch]$Android,

  [switch]$SkipWasm
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not $SkipWasm) {
  pnpm run wasm
}

if ($Android) {
  cargo tauri android dev
}
else {
  cargo tauri dev
}
