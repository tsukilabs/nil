// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { timeout } from '@tb-dev/utils';
import { handleError } from '@/lib/error';
import { useUserStore } from '@/stores/user';
import { Entity } from '@/core/entity/abstract';
import { exit } from '@tauri-apps/plugin-process';

async function joinGame(options: {
  worldId: NonNullable<ClientOptions['worldId']>;
  worldPassword: ClientOptions['worldPassword'];
  playerId: NonNullable<ClientOptions['playerId']>;
}) {
  await NIL.world.setId(options.worldId);

  const { playerId } = options;
  if (await commands.playerExists(playerId)) {
    // TODO: what if the player is already active?
    const status = await commands.getPlayerStatus(playerId);
    if (status === 'inactive') {
      await commands.setPlayerStatus('active');
    }
  }
  else {
    const playerOptions: PlayerOptions = { id: playerId };
    await commands.spawnPlayer(playerOptions, options.worldPassword);
  }

  await NIL.player.setId(playerId);
  await NIL.city.setCoord();
  await NIL.update();

  await go('city');
}

export async function joinLocalGame(options: {
  serverAddr: NonNullable<ClientOptions['serverAddr']>;
  worldId?: ClientOptions['worldId'];
  playerId: NonNullable<ClientOptions['playerId']>;
}) {
  await commands.updateClient({
    serverAddr: options.serverAddr,
    worldId: options.worldId,
    playerId: options.playerId,
  });

  options.worldId ??= await commands.getLocalServerWorldId();

  if (options.worldId) {
    return joinGame({
      worldId: options.worldId,
      worldPassword: null,
      playerId: options.playerId,
    });
  }
  else {
    throw new Error('Missing world id');
  }
}

export async function joinRemoteGame(options: {
  worldId: NonNullable<ClientOptions['worldId']>;
  worldPassword: ClientOptions['worldPassword'];
  authorizationToken: NonNullable<ClientOptions['authorizationToken']>;
}) {
  const playerId = await commands.validateToken(options.authorizationToken);
  if (playerId) {
    await commands.updateClient({
      serverAddr: { kind: 'remote' },
      worldId: options.worldId,
      worldPassword: options.worldPassword,
      authorizationToken: options.authorizationToken,
    });

    return joinGame({
      worldId: options.worldId,
      worldPassword: options.worldPassword,
      playerId,
    });
  }
  else {
    throw new Error('Invalid token');
  }
}

export async function hostLocalGame(options: {
  playerOptions: PlayerOptions;
  worldOptions: WorldOptions;
}) {
  const server = await commands.startServerWithOptions(options.worldOptions);
  const serverAddr: ServerAddr = { kind: 'local', addr: server.addr };
  return joinLocalGame({
    serverAddr,
    worldId: server.worldId,
    playerId: options.playerOptions.id,
  });
}

export async function hostLocalGameWithSavedata(options: {
  path: string;
  playerOptions: PlayerOptions;
}) {
  const server = await commands.startServerWithSavedata(options.path);
  const serverAddr: ServerAddr = { kind: 'local', addr: server.addr };
  return joinLocalGame({
    serverAddr,
    worldId: server.worldId,
    playerId: options.playerOptions.id,
  });
}

export async function hostRemoteGame(options: {
  worldOptions: WorldOptions;
  worldPassword: ClientOptions['worldPassword'];
  worldDescription: Option<string>;
  authorizationToken: NonNullable<ClientOptions['authorizationToken']>;
}) {
  const worldId = await commands.createRemoteWorld({
    options: options.worldOptions,
    password: options.worldPassword,
    description: options.worldDescription,
  });

  return joinRemoteGame({
    worldId,
    worldPassword: options.worldPassword,
    authorizationToken: options.authorizationToken,
  });
}

export async function leaveGame(options?: {
  navigate?: boolean;
  clearBrowsingData?: boolean;
}) {
  try {
    // We must check whether the server is remote
    // BEFORE stopping the client, because stopping
    // the client marks the server as remote.
    const isRemote = await commands.isRemote();

    Entity.dispose();
    await commands.stopClient();
    await commands.stopServer();

    if (options?.navigate ?? true) {
      const { isAuthorizationTokenValid } = useUserStore();
      if (
        isRemote &&
        location.pathname.startsWith('/game/') &&
        await timeout(isAuthorizationTokenValid, 5000)
      ) {
        await go('lobby');
      }
      else {
        await go('home');
      }
    }

    if (options?.clearBrowsingData ?? true) {
      await commands.clearAllBrowsingData();
    }
  }
  catch (err) {
    handleError(err);
  }
}

export async function exitGame() {
  await leaveGame({ navigate: false });
  await exit(0);
}
