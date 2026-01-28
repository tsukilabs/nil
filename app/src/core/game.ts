// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { Entity } from '@/core/entity/abstract';
import { exit } from '@tauri-apps/plugin-process';

async function joinGame(worldId: WorldId, playerOptions: PlayerOptions) {
  await NIL.world.setId(worldId);

  const playerId = playerOptions.id;
  if (await commands.playerExists(playerId)) {
    // TODO: what if the player is already active?
    const status = await commands.getPlayerStatus(playerId);
    if (status === 'inactive') {
      await commands.setPlayerStatus('active');
    }
  }
  else {
    await commands.spawnPlayer(playerOptions);
  }

  await NIL.player.setId(playerId);
  await NIL.city.setCoord();
  await NIL.update();

  await go('city');
}

export async function joinLocalGame(
  serverAddr: ServerAddr,
  worldId: Option<WorldId>,
  playerOptions: PlayerOptions,
) {
  await commands.updateClient({
    serverAddr,
    worldId,
    playerId: playerOptions.id,
    playerPassword: playerOptions.password,
  });

  worldId ??= await commands.getLocalServerWorldId();
  if (!worldId) {
    throw new Error('Missing world id');
  }

  return joinGame(worldId, playerOptions);
}

export async function joinRemoteGame(
  serverAddr: ServerAddr,
  worldId: WorldId,
  playerOptions: PlayerOptions,
) {
  await commands.updateClient({
    serverAddr,
    worldId,
    playerId: playerOptions.id,
    playerPassword: playerOptions.password,
  });

  return joinGame(worldId, playerOptions);
}

export async function hostLocalGame(player: PlayerOptions, world: WorldOptions) {
  const server = await commands.startServerWithOptions(world);
  const serverAddr: ServerAddr = { kind: 'local', addr: server.addr };
  await joinLocalGame(serverAddr, server.worldId, player);
}

export async function hostLocalGameWithSavedata(path: string, player: PlayerOptions) {
  const server = await commands.startServerWithSavedata(path);
  const serverAddr: ServerAddr = { kind: 'local', addr: server.addr };
  await joinLocalGame(serverAddr, server.worldId, player);
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
