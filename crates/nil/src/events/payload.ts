import type { Turn } from '@/types/turn';
import type { Player } from '@/types/player';

export type PlayerJoinedPayload = {
  kind: 'player-joined';
  player: Player;
};

export type TurnUpdatedPayload = {
  kind: 'turn-updated';
  turn: Turn;
};
