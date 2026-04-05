// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface Ethics {
  readonly power: EthicPowerAxis;
  readonly truth: EthicTruthAxis;
}

export type EthicPowerAxis = 'militarist' | 'fanatic-militarist' | 'pacifist' | 'fanatic-pacifist';

export type EthicTruthAxis =
  | 'materialist'
  | 'fanatic-materialist'
  | 'spiritualist'
  | 'fanatic-spiritualist';
