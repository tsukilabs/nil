// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Report_ {
  readonly id: ReportId;
  readonly round: RoundId;
  readonly time: string;
}

type ReportId = string;

type ReportKind = ReportKindBattle | ReportKindSupport;

interface ReportKindBattle {
  readonly kind: 'battle';
  readonly report: BattleReport;
}

interface ReportKindSupport {
  readonly kind: 'support';
  readonly report: SupportReport;
}

interface BattleReport extends Report_ {
  readonly attacker: Ruler;
  readonly defender: Ruler;
  readonly origin: Coord;
  readonly destination: Coord;
  readonly result: BattleResult;
  readonly hauledResources: Resources;
}

interface SupportReport extends Report_ {
  readonly sender: Ruler;
  readonly receiver: Ruler;
  readonly origin: Coord;
  readonly destination: Coord;
  readonly personnel: ArmyPersonnel;
}
