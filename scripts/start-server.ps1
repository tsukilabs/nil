# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('Url')]
  [switch]$DatabaseUrl
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$Command = 'cargo run -p nil-server --'

if ($DatabaseUrl) {
  $Command += " --database-url $DatabaseUrl" 
}

Invoke-Expression $Command.Trim()
