# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('A')]
  [string[]]$Allow,

  [Alias('U')]
  [switch]$Unused
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$ClippyCmd = 'cargo clippy --workspace --'

if ($Unused -and -not ($Allow -contains 'unused')) {
  $Allow += 'unused'
}

foreach ($Lint in $Allow) {
  $ClippyCmd += " -A $Lint"
}

Invoke-Expression $ClippyCmd
