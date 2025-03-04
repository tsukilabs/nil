import { go } from '@/router';
import { nextTick } from 'vue';
import { until } from '@vueuse/core';
import { Entity } from '@/core/entity';
import * as commands from '@/commands';
import { exit } from '@tauri-apps/plugin-process';

export async function joinGame(options: JoinOptions) {
  await commands.startClient(options.serverAddr);
  await commands.spawnPlayer(options.player);

  NIL.player.setId(options.player.id);
  await updateGame();

  if (await commands.isRoundIdle()) {
    go('lobby');
  } else {
    const { coord } = NIL.village.refs();
    await until(coord).toBeTruthy();
    go('village');
  }
}

export async function hostGame(options: HostOptions) {
  const addr = await commands.startServer(options.world);
  await joinGame({ player: options.player, serverAddr: addr.asLocal() });
}

export async function updateGame() {
  await Promise.all([NIL.round.update(), NIL.player.update()]);
  await nextTick();
}

export async function leaveGame() {
  await commands.stopClient();
  await commands.stopServer();

  Entity.clear();
  go('home');
}

export async function exitGame() {
  await leaveGame();
  await exit(0);
}
