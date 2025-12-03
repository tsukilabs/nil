# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('A')]
  [switch]$Android,
  
  [Alias('P')]
  [switch]$Preview,

  [Alias('W')]
  [switch]$Wasm
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($Wasm) {
  pnpm run wasm -Release
}

$BuildCmd = if ($Android) {
  'cargo tauri android build --apk true'
}
else {
  'cargo tauri build'
}

if (-not $Android) {
  if ($Preview) {
    $BuildCmd += ' --no-bundle -- --profile preview'
  }
  else {
    # We should stop using the `release-bin` profile once `wasm-pack` is updated.
    $BuildCmd += ' -- --profile release-bin'
  }
}


Invoke-Expression $BuildCmd
