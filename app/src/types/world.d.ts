// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface WorldOptions {
  readonly name: string;
  readonly size: number;
  readonly locale: Locale;
  readonly allowCheats: bool;
}

interface WorldConfig {
  readonly id: WorldId;
  readonly name: string;
  readonly locale: Locale;
  readonly allowCheats: bool;
}

type WorldId = string;

type Locale = 'en-US' | 'pt-BR';

interface WorldStats {
  readonly infrastructure: InfrastructureStats;
}
