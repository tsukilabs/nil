# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('R')]
  [switch]$Release,

  [Alias('P')]
  [switch]$Publish,

  [Alias('I')]
  [switch]$Install
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($Install) {
  rustup target add wasm32-unknown-unknown
  cargo install wasm-bindgen-cli
  cargo install wasm-pack
}

function Build {
  param ([string] $Crate)
  
  $Path = "crates/$Crate/pkg"

  if (Test-Path -Path $Path) {
    Get-ChildItem -Path $Path -Recurse | Remove-Item -Recurse
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
    $CurrentVersion = Get-Content -Path 'package.json' -Raw
    | ConvertFrom-Json
    | Select-Object -ExpandProperty 'version'

    $Request = @{
      Uri     = "https://registry.npmjs.org/@tsukilabs/$Crate"
      Headers = @{
        'Accept' = 'application/vnd.npm.install-v1+json'
      }
    }

    $Versions = (Invoke-RestMethod @Request).versions

    if (-not ($Versions.PSObject.Properties.Name -contains $CurrentVersion)) {
      $Extension = $('*.js', '*.ts')
      $Files = Get-ChildItem -Path $Path -Recurse -Name -Include $Extension 
      | ForEach-Object { Join-Path -Path $Path -ChildPath $_ }

      foreach ($File in $Files) {
        $Contents = "// Copyright (C) Call of Nil contributors`n"
        $Contents += "// SPDX-License-Identifier: AGPL-3.0-only`n"
        $Contents += "`n$(Get-Content -Path $File -Raw)"

        Set-Content -Path $File -Value $Contents
      }

      Invoke-Expression "pnpm -F @tsukilabs/$Crate publish --access public --no-git-checks"
    }
  }
}

Build -Crate 'nil-continent'
