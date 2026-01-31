# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [switch]$Android,
  [string]$Device,
  [switch]$Wasm,
  [switch]$Remote,
  [switch]$Verbose,
  [switch]$LogTowerHttp
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($Wasm) {
  pnpm run wasm
}

if ($Remote) {
  $Env:NIL_REMOTE_SERVER_ADDR = 'tsukilabs.dev.br/nil'
}

if ($LogTowerHttp -or $Verbose) {
  $Env:NIL_LOG_TOWER_HTTP = 'true'
}

if ($Android) {
  $Device = if ($Device) { `"$($Device.Trim())`" } else { '' }
  Invoke-Expression "cargo tauri android dev $Device".Trim()
}
else {
  cargo tauri dev
}
