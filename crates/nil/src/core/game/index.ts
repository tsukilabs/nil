import { go } from '@/router';
import { until } from '@vueuse/core';
import * as commands from '@/commands';
import { HandleError } from '@/lib/error';
import { PlayerImpl } from '@/core/player';
import type { Option } from '@tb-dev/utils';
import { VillageImpl } from '@/core/village';
import { SocketAddrV4 } from '@/lib/net/addr';
import { provide, tryInject } from '@/lib/app';
import type { WorldConfig } from '@/types/world';
import { exit } from '@tauri-apps/plugin-process';
import type { PlayerConfig } from '@/types/player';
import type { CoordImpl } from '@/core/village/coord';
import { computedAsync } from '@/composables/computed-async';
import { computed, effectScope, type InjectionKey, shallowRef, watch } from 'vue';

export class Game {
  private readonly player = shallowRef<Option<PlayerImpl>>();
  private readonly playerId = computed(() => this.player.value?.id ?? null);

  private readonly coord = shallowRef<Option<CoordImpl>>();
  private readonly village = computedAsync(null, () => {
    const coord = this.coord.value;
    return coord ? VillageImpl.load(coord) : null;
  });

  private constructor() {
    watch(this.player, (it) => this.onPlayerUpdate(it));
  }

  @HandleError({ async: true })
  private async join(server: SocketAddrV4, player: PlayerConfig) {
    await commands.startClient(server);
    const playerId = await commands.spawnPlayer(player);
    this.player.value = await PlayerImpl.load(playerId);

    await until(this.coord).toBeTruthy();
    go('village');
  }

  @HandleError({ async: true })
  private async host(world: WorldConfig, player: PlayerConfig) {
    const info = await commands.startServer(world);
    const addr = SocketAddrV4.parse(`127.0.0.1:${info.port}`);
    await this.join(addr, player);
  }

  private async updatePlayer() {
    if (this.playerId.value) {
      this.player.value = await PlayerImpl.load(this.playerId.value);
    }
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
    this.player.value = null;
    this.coord.value = null;
  }

  @HandleError({ async: true })
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
      playerId: game.playerId,
      updatePlayer: game.updatePlayer.bind(game),
      village: game.village,
    };
  }
}
