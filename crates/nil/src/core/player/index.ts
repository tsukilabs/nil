import * as commands from '@/commands';
import type { Coord } from '@/types/world';
import type { Player, PlayerId } from '@/types/player';

export class PlayerImpl implements Player {
  public readonly id: string;
  public readonly name: string;
  public readonly villages: Coord[];

  private constructor(player: Player, villages: Coord[]) {
    this.id = player.id;
    this.name = player.name;
    this.villages = villages;
  }

  public static async create(id: PlayerId) {
    const player = await commands.getPlayer(id);
    const villages = await commands.getPlayerVillages(id);
    return new PlayerImpl(player, villages);
  }
}
