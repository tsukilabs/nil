import type { WorldOptions } from './world';
import type { PlayerOptions } from './player';

export type HostOptions = {
  readonly player: PlayerOptions;
  readonly world: WorldOptions;
};

export type ServerInfo = {
  readonly port: number;
};
