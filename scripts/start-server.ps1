# Copyright (C) Call of Nil contributors
# SPDX-License-Identifier: AGPL-3.0-only

param(
  [Alias('V')]
  [switch]$Verbose,

  [switch]$LogTowerHttp
)

if ($LogTowerHttp -or $Verbose) {
  $Env:NIL_LOG_TOWER_HTTP = 'true'
}

cargo run -p nil-server
