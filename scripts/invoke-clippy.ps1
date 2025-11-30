# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('D')]
  [switch]$AllowDeadCode
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$ClippyCmd = 'cargo clippy --workspace --'

if ($AllowDeadCode) {
  $Lints = @('dead_code', 'unused_imports', 'unused_variables')

  foreach ($Lint in $Lints) {
    $ClippyCmd += " -A $Lint"
  }
}

Invoke-Expression $ClippyCmd
