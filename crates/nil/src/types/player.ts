export type Player = {
  readonly id: PlayerId;
  readonly name: string;
}

export type PlayerId = string;

export type PlayerConfig = {
  readonly name: string;
}
