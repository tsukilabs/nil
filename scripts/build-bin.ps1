# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [switch]$Android,
  [switch]$Preview,
  [switch]$OpenPreview,
  [switch]$Wasm,
  [string]$TargetDir,
  [switch]$Kanata
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
  $Path = './app/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk'
  $Version = Get-Content -Path 'package.json' -Raw
  | ConvertFrom-Json
  | Select-Object -ExpandProperty 'version'

  $Name = "call-of-nil-$Version.apk"

  if ($Kanata -and (Get-Command 'kanata' -ErrorAction SilentlyContinue)) {
    kanata add --path "`"$Path`"" --name "`"$Name`""
  }

  if ($TargetDir) {
    if ($IsWindows -and ($TargetDir.ToLower() -eq 'desktop')) {
      $TargetDir = [Environment]::GetFolderPath('Desktop')
    }

    Copy-Item $Path -Destination "$TargetDir/$Name"
  }
}

if ($Preview -and $OpenPreview) {
  Invoke-Item './target/preview/nil.exe'
}
