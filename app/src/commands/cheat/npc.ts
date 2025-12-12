// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { getCityOwner } from '@/commands/city';

export async function cheatGetEthics(ruler: Ruler) {
  return invoke<Option<Ethics>>('cheat_get_ethics', { req: { ruler } });
}

export async function cheatGetBotEthics(id: BotId) {
  return cheatGetEthics({ kind: 'bot', id });
}

export async function cheatGetOwnerEthics(coord: ContinentKey) {
  return cheatGetEthics(await getCityOwner(coord));
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
  await invoke('cheat_set_bot_ethics', { req: { id, ethics } });
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

  return invoke<BotId>('cheat_spawn_bot', { req: { name } });
}
