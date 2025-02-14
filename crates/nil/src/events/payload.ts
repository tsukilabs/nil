import type { Player } from '@/types/player';
import type { RoundState } from '@/types/round';

export type PlayerJoinedPayload = {
  kind: 'player-joined';
  player: Player;
};

export type RoundUpdatedPayload = {
  kind: 'round-updated';
  turn: RoundState;
};
