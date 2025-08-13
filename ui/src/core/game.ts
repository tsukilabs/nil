// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { Entity } from '@/core/entity/abstract';
import { exit } from '@tauri-apps/plugin-process';
import type { SocketAddrV4 } from '@/lib/net/addr-v4';

export async function joinGame(player: PlayerOptions, serverAddr: SocketAddrV4) {
  const id = player.id;
  await commands.startClient(player.id, serverAddr);

  if (await commands.playerExists(id)) {
    // TODO: what if the player is already active?
    const status = await commands.getPlayerStatus(id);
    if (status === 'inactive') {
      await commands.setPlayerStatus('active');
    }
  }
  else {
    await commands.spawnPlayer(player);
  }

  await NIL.player.setId(id);
  await NIL.village.setCoord();
  await NIL.update();

  await go('village');
}

export async function hostGame(player: PlayerOptions, world: WorldOptions) {
  const addr = await commands.startServerWithOptions(world);
  await joinGame(player, addr.asLocal());
}

export async function hostWithSavedata(path: string, player: PlayerOptions) {
  const addr = await commands.startServerWithSavedata(path);
  await joinGame(player, addr.asLocal());
}

export async function leaveGame() {
  try {
    Entity.dispose();
    await commands.stopClient();
    await commands.stopServer();
  }
  catch (err) {
    handleError(err);
  }
  finally {
    await go('home');
  }
}

export async function exitGame() {
  await leaveGame();
  await exit(0);
}
