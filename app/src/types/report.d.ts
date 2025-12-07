// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Report_ {
  readonly timestamp: string;
}

interface BattleReport extends Report_ {
  readonly attacker: Ruler;
  readonly defenders: readonly Ruler[];
  readonly hauledResources: Resources;
  readonly result: BattleResult;
}
