import { go } from '@/router';
import { until } from '@vueuse/core';
import * as commands from '@/commands';
import { PlayerImpl } from '@/core/player';
import type { Option } from '@tb-dev/utils';
import { maybe } from '@/composables/maybe';
import { VillageImpl } from '@/core/village';
import { provide, tryInject } from '@/lib/app';
import type { PlayerId } from '@/types/player';
import type { RoundState } from '@/types/round';
import { SocketAddrV4 } from '@/lib/net/addr-v4';
import { exit } from '@tauri-apps/plugin-process';
import type { HostOptions } from '@/types/server';
import type { JoinOptions } from '@/types/client';
import { asyncRef } from '@/composables/async-ref';
import type { CoordImpl } from '@/core/village/coord';
import { asyncComputed } from '@/composables/async-computed';
import { effectScope, type InjectionKey, ref, shallowRef, watch } from 'vue';

export class Game {
  private readonly round = shallowRef<Option<RoundState>>();
  private readonly playerId = ref<Option<PlayerId>>();
  private readonly player = asyncRef(null, async () => {
    return maybe(this.playerId, PlayerImpl.load.bind(PlayerImpl));
  });

  private readonly coord = shallowRef<Option<CoordImpl>>();
  private readonly village = asyncComputed(null, async () => {
    return maybe(this.coord, VillageImpl.load.bind(VillageImpl));
  });

  private constructor() {
    watch(this.playerId, () => this.updatePlayer());
    watch(this.player, (it) => this.onPlayerUpdate(it));
  }

  private async join(options: JoinOptions) {
    await commands.startClient(options.server);
    await commands.spawnPlayer(options.player);
    this.playerId.value = options.player.id;
    await this.update();

    await until(this.coord).toBeTruthy();
    go('village');
  }

  private async host(options: HostOptions) {
    const info = await commands.startServer(options.world);
    const server = SocketAddrV4.parse(`127.0.0.1:${info.port}`);
    await this.join({ player: options.player, server });
  }

  private async update() {
    await Promise.all([this.updateRound(), this.updatePlayer()]);
  }

  private async updatePlayer() {
    await this.player.execute();
  }

  private async updateRound() {
    this.round.value = await commands.getRoundState();
  }

  private onPlayerUpdate(player: Option<PlayerImpl>) {
    if (player && (!this.coord.value || !player.has(this.coord.value))) {
      this.coord.value = player.villages.at(0);
    } else if (!player) {
      this.coord.value = null;
    }
  }

  private async leave() {
    await commands.stopClient();
    await commands.stopServer();

    go('home');
    this.playerId.value = null;
    this.coord.value = null;
  }

  private async exit() {
    await this.leave();
    await exit(0);
  }

  static readonly #scope = effectScope(/* detached */ true);
  static readonly #symbol: InjectionKey<Game> = Symbol('game');

  private static create() {
    return this.#scope.run(() => {
      const game = new Game();
      provide(this.#symbol, game);
      return game;
    });
  }

  public static use() {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const game = tryInject(this.#symbol) ?? this.create()!;
    return {
      exit: game.exit.bind(game),
      host: game.host.bind(game),
      join: game.join.bind(game),
      leave: game.leave.bind(game),
      player: game.player as Readonly<typeof game.player>,
      update: game.update.bind(game),
      updatePlayer: game.updatePlayer.bind(game),
      village: game.village,
    };
  }
}
