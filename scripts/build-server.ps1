# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [switch]$Release,
  [string]$TargetDir
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

cargo build --profile release-server --package nil-server

$File = if ($IsWindows) { 'nil-server.exe' } else { 'nil-server' }
$Path = (Resolve-Path "./target/release-server/$File").ToString()

if ($TargetDir) {
  if ($IsWindows -and ($TargetDir.ToLower() -eq 'desktop')) {
    $TargetDir = [Environment]::GetFolderPath('Desktop')
  }

  Copy-Item $Path -Destination (Resolve-Path $TargetDir).ToString()
}

if ($Release -and $IsLinux) {
  $Version = Get-Content -Path 'package.json' -Raw
  | ConvertFrom-Json
  | Select-Object -ExpandProperty 'version'

  $AssetName = "Call.of.Nil_$($Version)_server"

  Rename-Item -Path $Path -NewName $AssetName

  $Path = (Resolve-Path "./target/release-server/$AssetName").ToString()

  $TagName = gh release view --json tagName -R 'tsukilabs/nil'
  | ConvertFrom-Json 
  | Select-Object -ExpandProperty 'tagName'

  gh release upload --clobber $TagName $Path -R 'tsukilabs/nil'

  $Token = [Environment]::GetEnvironmentVariable('TSUKILABS_TOKEN')
  $WebhookUrl = [Environment]::GetEnvironmentVariable('NIL_DISCORD_WEBHOOK_URL')

  if ($Token) {
    $Uri = 'http://tsukilabs.dev.br/release/nil'
    $Headers = @{
      Authorization = "Bearer $Token"
    }

    Invoke-WebRequest -Uri $Uri -Headers $Headers | Out-Null

    if ($WebhookUrl) {
      $AssetUrl = gh release view $TagName --json assets -R 'tsukilabs/nil'
      | ConvertFrom-Json 
      | Select-Object -ExpandProperty 'assets'
      | Where-Object { $_.name -eq $AssetName }
      | Select-Object -ExpandProperty 'url'
      
      $Message = "Server updated to v$Version"
      $Message += "`n<https://github.com/tsukilabs/nil/releases/tag/$TagName>"
      $Message += "`n<$AssetUrl>"

      $Body = @{
        content = $Message
      }

      Invoke-WebRequest -Uri $WebhookUrl -Method Post -Body $Body | Out-Null
    }
  }
}
