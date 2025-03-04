export type PlayerJoinedPayload = {
  kind: 'player-joined';
  player: Player;
};

export type PlayerLeftPayload = {
  kind: 'player-left';
  player: Player;
};

export type RoundUpdatedPayload = {
  kind: 'round-updated';
  turn: RoundState;
};
