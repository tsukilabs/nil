# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [switch]$Preview,
  [switch]$SkipWasm
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not $SkipWasm) {
  pnpm run wasm
}

$BuildCmd = 'cargo tauri build'

if ($Preview) {
  $BuildCmd += ' --no-bundle -- --profile preview'
}
else {
  $BuildCmd += ' -- --profile release-bin'
}

Invoke-Expression $BuildCmd
