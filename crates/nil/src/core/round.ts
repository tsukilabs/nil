import { shallowRef } from 'vue';
import * as commands from '@/commands';
import { Entity } from '@/core/entity';
import type { Option } from '@tb-dev/utils';

export class Round extends Entity {
  private readonly state = shallowRef<Option<RoundState>>();

  public isIdle() {
    return this.state.value?.phase.kind === 'idle';
  }

  public async update() {
    this.state.value = await commands.getRoundState();
  }

  public static use() {
    return super.get(Round) as Round;
  }

  public static refs() {
    const instance = this.use();
    return {
      state: instance.state as Readonly<typeof instance.state>,
    };
  }

  public static isIdle() {
    return this.use().isIdle();
  }

  public static update() {
    return this.use().update();
  }

  public static init() {
    if (!Object.hasOwn(window.NIL, 'round')) {
      const round: (typeof window.NIL)['round'] = {
        isIdle: Round.isIdle.bind(Round),
        refs: Round.refs.bind(Round),
        update: Round.update.bind(Round),
        use: Round.use.bind(Round),
      };

      Object.defineProperty(window.NIL, 'round', {
        configurable: false,
        enumerable: true,
        value: round,
        writable: false,
      });
    }
  }
}
