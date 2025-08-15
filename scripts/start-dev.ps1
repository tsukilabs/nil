# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('A')]
  [switch]$Android,

  [Alias('D')]
  [string]$Device = '',

  [Alias('S')]
  [switch]$SkipWasm
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not $SkipWasm) {
  pnpm run wasm
}

if ($Android) {
  Invoke-Expression "cargo tauri android dev `"$($Device.Trim())`"".Trim()
}
else {
  cargo tauri dev
}
