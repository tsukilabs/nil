import type { PlayerId } from './player';

export interface Turn {
  readonly id: TurnId;
  readonly state: TurnState;
}

export type TurnId = number;

export type TurnState = TurnStateIdle | TurnStateWaiting;

export interface TurnStateIdle {
  readonly kind: 'idle';
}

export interface TurnStateWaiting {
  readonly kind: 'waiting';
  readonly player: PlayerId;
}
