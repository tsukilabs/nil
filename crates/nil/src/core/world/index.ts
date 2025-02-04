import { go } from '@/router';
import { until } from '@vueuse/core';
import * as commands from '@/commands';
import { HandleError } from '@/lib/error';
import { PlayerImpl } from '@/core/player';
import type { Option } from '@tb-dev/utils';
import { SocketAddrV4 } from '@/lib/net/addr';
import type { PlayerConfig } from '@/types/player';
import type { Coord, WorldConfig } from '@/types/world';
import { fallibleInject, provide, runWithContext } from '@/lib/app';
import { computed, type InjectionKey, shallowRef, watch } from 'vue';

const worldSymbol: InjectionKey<World> = Symbol('world');

export class World {
  private readonly player = shallowRef<Option<PlayerImpl>>();
  private readonly playerId = computed(() => this.player.value?.id ?? null);

  private readonly currentVillage = shallowRef<Option<Coord>>();

  private constructor() {
    runWithContext(() => {
      watch(this.player, this.onPlayerUpdate.bind(this));
    });
  }

  @HandleError({ async: true })
  public async join(server: SocketAddrV4, player: PlayerConfig) {
    await commands.startClient(server);
    const playerId = await commands.spawnPlayer(player);
    this.player.value = await PlayerImpl.load(playerId);

    await until(this.currentVillage).toBeTruthy();
    go('village');
  }

  @HandleError({ async: true })
  public async host(world: WorldConfig, player: PlayerConfig) {
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
    if (player && (!this.currentVillage.value || !player.has(this.currentVillage.value))) {
      this.currentVillage.value = player.villages.at(0);
    } else if (!player) {
      this.currentVillage.value = null;
    }
  }

  public static use() {
    let world = fallibleInject(worldSymbol);
    if (!world) {
      world = new World();
      provide(worldSymbol, world);
    }

    return {
      player: world.player as Readonly<typeof world.player>,
      playerId: world.playerId,
      currentVillage: world.currentVillage as Readonly<typeof world.currentVillage>,
      join: world.join.bind(world),
      host: world.host.bind(world),
      updatePlayer: world.updatePlayer.bind(world),
    };
  }
}
