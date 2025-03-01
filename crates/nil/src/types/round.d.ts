type RoundState = {
  readonly id: RoundId;
  readonly phase: RoundPhase;
};

type RoundId = number;

type RoundPhase = RoundPhaseIdle | RoundPhasePlayer;

type RoundPhaseIdle = {
  readonly kind: 'idle';
};

type RoundPhasePlayer = {
  readonly kind: 'player';
  readonly pending: readonly PlayerId[];
};
