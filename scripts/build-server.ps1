# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [string]$TargetDir
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

cargo build --profile release-server --package nil-server

if ($TargetDir) {
  $File = if ($IsWindows) { 'nil-server.exe' } else { 'nil-server' }
  Copy-Item "./target/release-server/$File" -Destination $TargetDir
}
