// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import * as dialog from '@/lib/dialog';
import { handleError } from '@/lib/error';
import { Entity } from '@/core/entity/abstract';
import { exit } from '@tauri-apps/plugin-process';
import type { SocketAddrV4 } from '@/lib/net/addr-v4';

export async function joinGame(options: { player: PlayerOptions; serverAddr: SocketAddrV4 }) {
  const id = options.player.id;
  await commands.startClient(options.player.id, options.serverAddr);

  if (await commands.playerExists(id)) {
    // TODO: what if the player is already active?
    const status = await commands.getPlayerStatus(id);
    if (status === 'inactive') {
      await commands.setPlayerStatus('active');
    }
  } else {
    await commands.spawnPlayer(options.player);
  }

  await NIL.player.setId(id);
  await NIL.village.setCoord();
  await NIL.update();

  await go('village');
}

export async function hostGame(options: { player: PlayerOptions; world: WorldOptions }) {
  const addr = await commands.startServerWithOptions(options.world);
  await joinGame({ player: options.player, serverAddr: addr.asLocal() });
}

export async function hostSavedGame(options: { path: string; player: PlayerOptions }) {
  const addr = await commands.startServerWithSavedata(options.path);
  await joinGame({ player: options.player, serverAddr: addr.asLocal() });
}

export async function leaveGame() {
  try {
    Entity.dispose();
    await commands.stopClient();
    await commands.stopServer();
  } catch (err) {
    handleError(err);
  } finally {
    await go('home');
  }
}

export async function exitGame() {
  await leaveGame();
  await exit(0);
}

export async function saveGame() {
  const path = await dialog.save({
    filters: [{ name: 'Nil', extensions: ['nil'] }],
  });

  if (path) {
    await commands.saveWorld(path);
  }
}
