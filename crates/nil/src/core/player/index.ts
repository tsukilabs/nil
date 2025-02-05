import * as commands from '@/commands';
import type { Coord } from '@/types/village';
import { CoordImpl } from '@/core/village/coord';
import type { Player, PlayerId } from '@/types/player';

export class PlayerImpl implements Player {
  public readonly id: string;
  public readonly name: string;
  public readonly villages: CoordImpl[];

  private constructor(player: Player, villages: Coord[]) {
    this.id = player.id;
    this.name = player.name;
    this.villages = villages.map((it) => CoordImpl.create(it));
  }

  public has(coord: Coord) {
    return this.villages.some((village) => village.is(coord));
  }

  public static async load(id: PlayerId) {
    const player = await commands.getPlayer(id);
    const villages = await commands.getPlayerVillages(id);
    return new PlayerImpl(player, villages);
  }
}
