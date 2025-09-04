# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run wasm -Release

function Publish {
  param ([string] $Package)
  
  Invoke-Expression "pnpm -F @tsukilabs/$Package publish --access public"
}

Publish -Package 'nil-continent'
