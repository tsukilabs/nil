# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [switch]$Miri
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$TestCmd = 'cargo test'

if ($Miri) {
  $TestCmd = 'cargo miri test --workspace --exclude nil-continent'
}

$TestCmd += ' --tests'

Invoke-Expression $TestCmd
