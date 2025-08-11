// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export function cheatGetBotResources(id: BotId) {
  return invoke<Resources>('cheat_get_bot_resources', { id });
}

export function cheatGetBotStorageCapacity(id: BotId) {
  return invoke<OverallStorageCapacity>('cheat_get_bot_storage_capacity', { id });
}

export function cheatGetPrecursorResources(id: PrecursorId) {
  return invoke<Resources>('cheat_get_precursor_resources', { id });
}

export function cheatGetPrecursorStorageCapacity(id: PrecursorId) {
  return invoke<OverallStorageCapacity>('cheat_get_precursor_storage_capacity', { id });
}

export function cheatSpawnBot(name?: Option<string>) {
  name ??= 'Bot';
  return invoke<BotId>('cheat_spawn_bot', { name });
}
