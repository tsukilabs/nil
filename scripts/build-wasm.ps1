# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('R')]
  [switch]$Release
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

function Build {
  param ([string] $Crate)
  
  if (Test-Path -Path "crates/$Crate/pkg" ) {
    Get-ChildItem "crates/$Crate/pkg" -Recurse | Remove-Item
  }

  $BuildCmd = "wasm-pack build crates/$Crate --reference-types"

  if ($Release) {
    $BuildCmd += ' --release'
  }
  else {
    $BuildCmd += ' --dev --no-opt'
  }

  Invoke-Expression $BuildCmd
}

Build -Crate 'nil-continent'
