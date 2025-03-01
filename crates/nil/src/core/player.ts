import * as commands from '@/commands';
import { CoordImpl } from '@/core/coord';

export class PlayerImpl implements Player {
  public readonly id: string;
  public readonly villages: readonly CoordImpl[];

  private constructor(player: Player, villages: readonly Coord[]) {
    this.id = player.id;
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
