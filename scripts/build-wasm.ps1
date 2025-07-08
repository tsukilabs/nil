# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

function Build {
  param ([string] $Crate)
  
  Get-ChildItem "crates/$Crate/pkg" -Recurse | Remove-Item

  wasm-pack build "crates/$Crate" --release --reference-types
}


Build -Crate 'nil-continent'
