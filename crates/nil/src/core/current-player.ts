import { ref } from 'vue';
import { Entity } from '@/core/entity';
import { PlayerImpl } from '@/core/player';
import { maybe } from '@/composables/maybe';
import type { Option } from '@tb-dev/utils';
import { asyncRef } from '@/composables/async-ref';

export class CurrentPlayer extends Entity {
  private readonly id = ref<Option<PlayerId>>();
  private readonly player = asyncRef(null, async () => {
    return maybe(this.id, (id) => PlayerImpl.load(id));
  });

  constructor() {
    super();

    this.watch(this.id, () => this.update());
  }

  public async update() {
    await this.player.execute();
  }

  public static use() {
    return super.get(CurrentPlayer) as CurrentPlayer;
  }

  public static refs() {
    const instance = this.use();
    return {
      id: instance.id as Readonly<typeof instance.id>,
      player: instance.player,
    };
  }

  public static update() {
    return this.use().update();
  }

  public static setId(id: PlayerId) {
    this.use().id.value = id;
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'player')) {
      const player: (typeof window.NIL)['player'] = {
        refs: CurrentPlayer.refs.bind(CurrentPlayer),
        setId: CurrentPlayer.setId.bind(CurrentPlayer),
        update: CurrentPlayer.update.bind(CurrentPlayer),
        use: CurrentPlayer.use.bind(CurrentPlayer),
      };

      Object.defineProperty(window.NIL, 'player', {
        configurable: false,
        enumerable: true,
        value: player,
        writable: false,
      });
    }
  }
}
