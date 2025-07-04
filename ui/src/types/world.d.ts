// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface WorldOptions {
  readonly name: string;
  readonly size: number;
}

interface WorldConfig {
  readonly name: string;
}

interface WorldStats {
  readonly infrastructure: InfrastructureStats;
}
