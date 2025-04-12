import { ref, type Ref } from 'vue';
import { Entity } from '@/core/entity';
import { PlayerImpl } from '@/core/player';
import type { Option } from '@tb-dev/utils';
import { asyncRef, maybe } from '@tb-dev/vue';

export class CurrentPlayer extends Entity {
  private readonly id = ref<Option<PlayerId>>();
  private readonly player: Ref<null | PlayerImpl>;
  private readonly updatePlayer: () => Promise<void>;

  constructor() {
    super();

    const player = asyncRef(null, async () => {
      return maybe(this.id, (id) => PlayerImpl.load(id));
    });

    this.player = player.state;
    this.updatePlayer = async () => {
      await player.execute();
    };

    this.watch(this.id, () => this.update());
  }

  public async update() {
    await this.updatePlayer();
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
