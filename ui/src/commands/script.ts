// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function addScript(script: AddScriptRequest) {
  return addScripts([script]).then(([id]) => id);
}

export function addScripts(scripts: AddScriptRequest[]) {
  return invoke<readonly ScriptId[]>('add_scripts', { scripts });
}

export async function executeScript(id: ScriptId) {
  const stdout = await invoke<Stdout>('execute_script', { id });
  if (__DEBUG_ASSERTIONS__) console.log(stdout.join('\n'));
  return stdout;
}

export async function executeScriptChunk(chunk: string) {
  const stdout = await invoke<Stdout>('execute_script_chunk', { chunk });
  if (__DEBUG_ASSERTIONS__) console.log(stdout.join('\n'));
  return stdout;
}

export function exportScript(dir: string, name: string, code: string) {
  return invoke<null>('export_script', { dir, name, code });
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

export function updateScript(script: Script) {
  return invoke<null>('update_script', { script });
}
