// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { getCityOwner } from '@/commands/city';
import type {
  CheatGetEthicsRequest,
  CheatSetBotEthicsRequest,
  CheatSpawnBotRequest,
} from '@/lib/request';

export async function cheatGetEthics(ruler: Ruler) {
  const req: CheatGetEthicsRequest = {
    world: NIL.world.getIdStrict(),
    ruler,
  };

  return invoke<Option<Ethics>>('cheat_get_ethics', { req });
}

export async function cheatGetBotEthics(id: BotId) {
  return cheatGetEthics({ kind: 'bot', id });
}

export async function cheatGetOwnerEthics(coord: ContinentKey) {
  const ruler = await getCityOwner(coord);
  return cheatGetEthics(ruler);
}

export async function cheatGetOwnerPowerEthic(coord: ContinentKey) {
  const ethics = await cheatGetOwnerEthics(coord);
  return ethics?.power ?? null;
}

export async function cheatGetOwnerTruthEthic(coord: ContinentKey) {
  const ethics = await cheatGetOwnerEthics(coord);
  return ethics?.truth ?? null;
}

export async function cheatGetPowerEthic(ruler: Ruler) {
  const ethics = await cheatGetEthics(ruler);
  return ethics?.power ?? null;
}

export async function cheatGetPrecursorEthics(id: PrecursorId) {
  return cheatGetEthics({ kind: 'precursor', id });
}

export async function cheatGetTruthEthic(ruler: Ruler) {
  const ethics = await cheatGetEthics(ruler);
  return ethics?.truth ?? null;
}

export async function cheatSetBotEthics(id: BotId, ethics: Ethics) {
  const req: CheatSetBotEthicsRequest = {
    world: NIL.world.getIdStrict(),
    id,
    ethics,
  };

  await invoke('cheat_set_bot_ethics', { req });
}

export async function cheatSetBotPowerEthic(id: BotId, ethic: EthicPowerAxis) {
  const ethics = await cheatGetBotEthics(id);
  if (ethics) {
    await cheatSetBotEthics(id, { power: ethic, truth: ethics.truth });
  }
}

export async function cheatSetBotTruthEthic(id: BotId, ethic: EthicTruthAxis) {
  const ethics = await cheatGetBotEthics(id);
  if (ethics) {
    await cheatSetBotEthics(id, { power: ethics.power, truth: ethic });
  }
}

export async function cheatSetOwnerBotEthics(coord: ContinentKey, ethics: Ethics) {
  const owner = await getCityOwner(coord);
  if (owner.kind === 'bot') {
    await cheatSetBotEthics(owner.id, ethics);
  }
}

export async function cheatSetOwnerBotPowerEthic(coord: ContinentKey, ethic: EthicPowerAxis) {
  const owner = await getCityOwner(coord);
  if (owner.kind === 'bot') {
    await cheatSetBotPowerEthic(owner.id, ethic);
  }
}

export async function cheatSetOwnerBotTruthEthic(coord: ContinentKey, ethic: EthicTruthAxis) {
  const owner = await getCityOwner(coord);
  if (owner.kind === 'bot') {
    await cheatSetBotTruthEthic(owner.id, ethic);
  }
}

export async function cheatSpawnBot(name?: Option<string>) {
  if (typeof name !== 'string' || name.length === 0) {
    name = `Bot ${globalThis.crypto.randomUUID()}`;
  }

  const req: CheatSpawnBotRequest = {
    world: NIL.world.getIdStrict(),
    name,
  };

  return invoke<BotId>('cheat_spawn_bot', { req });
}
