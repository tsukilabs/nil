# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [switch]$Android,
  [switch]$Preview,
  [switch]$OpenPreview,
  [switch]$Wasm,
  [string]$TargetDir,
  [switch]$Desktop
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($Wasm) {
  pnpm run wasm -Release
}

$BuildCmd = if ($Android) {
  'cargo tauri android build --apk true'
}
else {
  'cargo tauri build'
}

if (-not $Android) {
  if ($Preview) {
    $Env:NIL_MINIFY_SOURCE = 'false'
    $BuildCmd += ' --no-bundle -- --profile preview'
  }
  else {
    $BuildCmd += ' -- --release'
  }
}


Invoke-Expression $BuildCmd

if ($Android) {
  if ($IsWindows -and $Desktop) {
    $TargetDir = [Environment]::GetFolderPath('Desktop')
  }

  if ($TargetDir) {
    $Version = Get-Content -Path 'package.json' -Raw
    | ConvertFrom-Json
    | Select-Object -ExpandProperty 'version'

    $Params = @{
      Path        = 'app/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk'
      Destination = "$TargetDir/call-of-nil-$Version.apk"
    }

    Copy-Item @Params
  }
}



if ($Preview -and $OpenPreview) {
  Invoke-Item './target/preview/nil.exe'
}
