// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function executeScript(chunk: string) {
  return invoke<ScriptOutput>('execute_script', { chunk });
}

export async function executeScriptAt(path: string) {
  return invoke<ScriptOutput>('execute_script_at', { path });
}

export async function importScript(path: string) {
  await invoke('import_script', { path });
}

export async function importScripts(paths: readonly string[]) {
  await invoke('import_scripts', { paths });
}

export async function isScript(path: string) {
  return invoke<boolean>('is_script', { path });
}

export async function loadScripts() {
  return invoke<readonly Script[]>('load_scripts');
}

export async function scriptDir() {
  return invoke<string>('script_dir');
}
