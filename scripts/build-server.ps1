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

if ($Release -and ($IsWindows -or $IsLinux)) {
  $Version = Get-Content -Path 'package.json' -Raw
  | ConvertFrom-Json
  | Select-Object -ExpandProperty 'version'

  $Name = "Call.of.Nil_$($Version)_server"
  if ($IsWindows) {
    $Name += '.exe'
  }

  Rename-Item -Path $Path -NewName $Name
  
  $Path = (Resolve-Path "./target/release-server/$Name").ToString()

  $TagName = gh release view --json tagName -R 'tsukilabs/nil'
  | ConvertFrom-Json 
  | Select-Object -ExpandProperty 'tagName'

  gh release upload --clobber $TagName $Path -R 'tsukilabs/nil'

  if ($IsLinux) {
    $Token = [Environment]::GetEnvironmentVariable('TSUKILABS_TOKEN')
    $WebhookUrl = [Environment]::GetEnvironmentVariable('NIL_DISCORD_WEBHOOK_URL')

    if ($Token) {
      $Uri = 'http://tsukilabs.dev.br/release/nil'
      $Headers = @{
        Authorization = "Bearer $Token"
      }

      Invoke-WebRequest -Uri $Uri -Headers $Headers

      if ($WebhookUrl) {
        $Message = "Server updated to v$Version."
        $Message += "`nhttps://github.com/tsukilabs/nil/releases/tag/$TagName"

        $Body = @{
          content = $Message
        }

        Invoke-WebRequest -Uri $WebhookUrl -Method Post -Body $Body
      }
    }
  }
}
