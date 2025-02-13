import type { PlayerId } from './player';

export type Turn = {
  readonly id: TurnId;
  readonly state: TurnState;
}

export type TurnId = number;

export type TurnState = TurnStateIdle | TurnStateWaiting;

export type TurnStateIdle = {
  readonly kind: 'idle';
}

export type TurnStateWaiting = {
  readonly kind: 'waiting';
  readonly player: PlayerId;
}
