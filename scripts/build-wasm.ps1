# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('R')]
  [switch]$Release,

  [Alias('P')]
  [switch]$Publish
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

function Build {
  param ([string] $Crate)
  
  $Path = "crates/$Crate/pkg"

  if (Test-Path -Path $Path ) {
    Get-ChildItem -Path $Path -Recurse | Remove-Item
  }

  $BuildCmd = "wasm-pack build crates/$Crate --scope tsukilabs"
  $BuildCmd += if ($Release -or $Publish) {
    ' --release'
  }
  else {
    ' --dev --no-opt' 
  }

  Invoke-Expression $BuildCmd

  if ($Publish) {
    $Extension = $('*.js', '*.ts')
    $Files = Get-ChildItem -Path $Path -Recurse -Name -Include $Extension 
    | ForEach-Object { Join-Path -Path $Path -ChildPath $_ }

    foreach ($File in $Files) {
      $Contents = "// Copyright (C) Call of Nil contributors`n"
      $Contents += "// SPDX-License-Identifier: AGPL-3.0-only`n"
      $Contents += "`n$(Get-Content -Path $File -Raw)"

      Set-Content -Path $File -Value $Contents
    }

    Invoke-Expression "pnpm -F @tsukilabs/$Crate publish --access public"
  }
}

Build -Crate 'nil-continent'
