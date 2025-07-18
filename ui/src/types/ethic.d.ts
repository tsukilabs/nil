// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Ethics {
  readonly power: EthicPowerAxis;
  readonly truth: EthicTruthAxis;
}

type EthicPowerAxis = 'militarist' | 'pacifist';
type EthicTruthAxis = 'materialist' | 'spiritualist';
