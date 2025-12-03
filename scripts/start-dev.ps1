# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('A')]
  [switch]$Android,

  [Alias('D')]
  [string]$Device,

  [Alias('W')]
  [switch]$Wasm
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($Wasm) {
  pnpm run wasm
}

if ($Android) {
  $Device = if ($Device) { `"$($Device.Trim())`" } else { '' }
  Invoke-Expression "cargo tauri android dev $Device".Trim()
}
else {
  cargo tauri dev
}
