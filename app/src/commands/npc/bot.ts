// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type { GetBotCoordsRequest, GetPublicBotRequest } from '@/lib/request';

export async function getBotCoords(id: BotId) {
  const req: GetBotCoordsRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<readonly Coord[]>('get_bot_coords', { req });
}

export async function getPublicBot(id: BotId) {
  const req: GetPublicBotRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<PublicBot>('get_public_bot', { req });
}
