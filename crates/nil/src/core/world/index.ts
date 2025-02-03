import * as commands from '@/commands';
import { HandleError } from '@/lib/error';
import { PlayerImpl } from '@/core/player';
import type { Option } from '@tb-dev/utils';
import type { PlayerConfig } from '@/types/player';
import { fallibleInject, provide } from '@/lib/app';
import type { Coord, WorldConfig } from '@/types/world';
import { computed, type InjectionKey, shallowRef } from 'vue';

const worldSymbol: InjectionKey<World> = Symbol('world');

export class World {
  private readonly player = shallowRef<Option<PlayerImpl>>();
  private readonly playerId = computed(() => this.player.value?.id ?? null);

  private readonly currentVillage = shallowRef<Option<Coord>>();

  private constructor() {}

  @HandleError({ async: true })
  public async join(server: string, player: PlayerConfig) {
    await commands.startClient(server);
    const playerId = await commands.spawnPlayer(player);
    this.player.value = await PlayerImpl.create(playerId);
    console.log('Player joined:', this.player.value);
  }

  @HandleError({ async: true })
  public async host(world: WorldConfig, player: PlayerConfig) {
    await commands.startServer(world);
    await this.join('127.0.0.1', player);
  }

  private async updatePlayer() {
    if (this.playerId.value) {
      this.player.value = await PlayerImpl.create(this.playerId.value);
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
