// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Ethics {
  readonly power: EthicPowerAxis;
  readonly truth: EthicTruthAxis;
}

type EthicPowerAxis = 'militarist' | 'fanatic-militarist' | 'pacifist' | 'fanatic-pacifist';

type EthicTruthAxis =
  | 'materialist'
  | 'fanatic-materialist'
  | 'spiritualist'
  | 'fanatic-spiritualist';
