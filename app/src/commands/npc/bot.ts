// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  BotId,
  GetBotCoordsRequest,
  GetBotCoordsResponse,
  GetPublicBotRequest,
  GetPublicBotResponse,
  GetPublicBotsRequest,
  GetPublicBotsResponse,
} from '@/types/bindings';

export async function getBotCoords(id: BotId) {
  const req: GetBotCoordsRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetBotCoordsResponse>('get_bot_coords', { req });
}

export async function getPublicBot(id: BotId) {
  const req: GetPublicBotRequest = {
    world: NIL.world.getIdStrict(),
    id,
  };

  return invoke<GetPublicBotResponse>('get_public_bot', { req });
}

export async function getPublicBots() {
  const req: GetPublicBotsRequest = {
    world: NIL.world.getIdStrict(),
  };

  return invoke<GetPublicBotsResponse>('get_public_bots', { req });
}
