# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [switch]$Release,
  [string]$TargetDir
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

cargo build --profile release-server --package nil-server

$File = if ($IsWindows) { 'nil-server.exe' } else { 'nil-server' }
$Path = "./target/release-server/$File"

if ($TargetDir) {
  if ($IsWindows -and ($TargetDir.ToLower() -eq 'desktop')) {
    $TargetDir = [Environment]::GetFolderPath('Desktop')
  }

  Copy-Item $Path -Destination $TargetDir
}

if ($Release -and ($IsWindows -or $IsLinux)) {
  $Version = Get-Content -Path 'package.json' -Raw
  | ConvertFrom-Json
  | Select-Object -ExpandProperty 'version'

  $Name = "Call.of.Nil_$($Version)_server"
  if ($IsWindows) {
    $Name += '.exe'
  }

  Rename-Item -Path $Path -NewName $Name

  $TagName = gh release view --json tagName -R 'tsukilabs/nil'
  | ConvertFrom-Json 
  | Select-Object -ExpandProperty 'tagName'

  gh release upload $TagName $Path -R 'tsukilabs/nil' 
}
