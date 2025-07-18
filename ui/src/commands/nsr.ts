// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function fetchNsrReadme(id: string) {
  return invoke<string>('fetch_nsr_readme', { id });
}

export function fetchNsrRegistry() {
  return invoke<readonly NsrScript[]>('fetch_nsr_registry');
}

export function fetchNsrScript(id: string) {
  return invoke<string>('fetch_nsr_script', { id });
}
