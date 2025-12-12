// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { getPublicCity } from '@/commands/city';
import { CoordImpl } from '@/core/model/continent/coord';

export async function cheatGetEthics(ruler: Ruler) {
  return invoke<Option<Ethics>>('cheat_get_ethics', { req: { ruler } });
}

export async function cheatGetBotEthics(id: BotId) {
  return cheatGetEthics({ kind: 'bot', id });
}

export async function cheatGetOwnerEthics(coord: ContinentKey) {
  coord = CoordImpl.fromContinentKey(coord);
  const city = await getPublicCity(coord);
  return cheatGetEthics(city.owner);
}

export async function cheatGetPrecursorEthics(id: PrecursorId) {
  return cheatGetEthics({ kind: 'precursor', id });
}

export async function cheatSpawnBot(name?: Option<string>) {
  if (typeof name !== 'string' || name.length === 0) {
    name = `Bot ${globalThis.crypto.randomUUID()}`;
  }

  return invoke<BotId>('cheat_spawn_bot', { req: { name } });
}
