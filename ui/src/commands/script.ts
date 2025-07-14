// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function addScript(script: Script) {
  return addScripts([script]).then(([id]) => id);
}

export function addScripts(scripts: Script[]) {
  return invoke<readonly ScriptId[]>('add_scripts', { scripts });
}

export function exportScript(dir: string, script: Script) {
  return invoke<null>('export_script', { dir, script });
}

export function getScript(id: ScriptId) {
  return invoke<Script>('get_script', { id });
}

export function getScripts() {
  return invoke<Script[]>('get_scripts');
}

export function importScripts(paths: string[]) {
  return invoke<readonly ScriptId[]>('import_scripts', { paths });
}

export function removeScript(id: ScriptId) {
  return invoke<null>('remove_script', { id });
}

export function executeScript(id: ScriptId) {
  return invoke<null>('execute_script', { id });
}

export function updateScript(script: Script) {
  return invoke<null>('update_script', { script });
}
