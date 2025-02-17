import type { PlayerOptions } from './player';
import type { SocketAddrV4 } from '@/lib/net/addr-v4';

export type JoinOptions = {
  readonly player: PlayerOptions;
  readonly server: SocketAddrV4;
};
