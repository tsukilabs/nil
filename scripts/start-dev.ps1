# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('A')]
  [switch]$Android,

  [Alias('D')]
  [string]$Device,

  [Alias('S')]
  [switch]$SkipWasm
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not $SkipWasm) {
  pnpm run wasm
}

if ($Android) {
  $Device = if ($Device) { `"$($Device.Trim())`" } else { '' }
  Invoke-Expression "cargo tauri android dev $Device".Trim()
}
else {
  cargo tauri dev
}
