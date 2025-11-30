// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';

export async function getBotCoords(id: BotId) {
  return invoke<readonly Coord[]>('get_bot_coords', { id });
}

export async function getPublicBot(id: BotId) {
  return invoke<PublicBot>('get_public_bot', { id });
}
