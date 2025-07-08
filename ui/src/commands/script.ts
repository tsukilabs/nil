// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';
import { invoke } from '@tauri-apps/api/core';

export function addScript(script: Script) {
  return invoke<ScriptId>('add_script', { script });
}

export function addScripts(scripts: Script[]) {
  return invoke<readonly ScriptId[]>('add_scripts', { scripts });
}

export function exportScript(dir: string, script: Script) {
  return invoke<null>('export_script', { dir, script });
}

export function getScript(id: ScriptId) {
  return invoke<Option<Script>>('get_script', { id });
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
