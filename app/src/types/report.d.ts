// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Report_ {
  readonly id: ReportId;
  readonly timestamp: string;
}

type ReportKind = ReportKindBattle;

interface ReportKindBattle {
  readonly kind: 'battle';
  readonly report: BattleReport;
}

interface BattleReport extends Report_ {
  readonly attacker: Ruler;
  readonly defender: Ruler;
  readonly result: BattleResult;
  readonly city: PublicCity;
  readonly hauledResources: Resources;
}

type ReportId = string;
