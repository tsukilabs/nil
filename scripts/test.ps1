# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('M')]
  [switch]$Miri
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$TestCmd = if ($Miri) { 'cargo miri test' } else { 'cargo test' }

if ($Miri) {
  $Exclude = @('nil', 'nil-continent')
  foreach ($Crate in $Exclude) {
    $TestCmd += " --exclude $Crate"
  }
}

$TestCmd += ' --workspace --tests'

Invoke-Expression $TestCmd
