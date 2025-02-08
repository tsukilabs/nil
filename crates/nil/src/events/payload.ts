import type { Turn } from '@/types/turn';
import type { Player } from '@/types/player';

export interface PlayerJoinedPayload {
  kind: 'player-joined';
  player: Player;
}

export interface TurnUpdatedPayload {
  kind: 'turn-updated';
  turn: Turn;
}
