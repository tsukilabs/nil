// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function executeScript(chunk: string) {
  return invoke<ScriptOutput>('execute_script', { chunk });
}

export async function executeScriptAt(path: string) {
  return invoke<ScriptOutput>('execute_script_at', { path });
}
