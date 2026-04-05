// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import type { RoundId } from '@/types/core/round';
import type { Coord } from '@/types/core/continent';
import type { BattleResult } from '@/types/core/battle';
import type { Resources } from '@/types/core/resources';

export interface Report_ {
  readonly id: ReportId;
  readonly round: RoundId;
  readonly time: string;
}

export type ReportId = string;

export type ReportKind = ReportKindBattle | ReportKindSupport;

export interface ReportKindBattle {
  readonly kind: 'battle';
  readonly report: BattleReport;
}

export interface ReportKindSupport {
  readonly kind: 'support';
  readonly report: SupportReport;
}

export interface BattleReport extends Report_ {
  readonly attacker: Ruler;
  readonly defender: Ruler;
  readonly origin: Coord;
  readonly destination: Coord;
  readonly result: BattleResult;
  readonly hauledResources: Resources;
}

export interface SupportReport extends Report_ {
  readonly sender: Ruler;
  readonly receiver: Ruler;
  readonly origin: Coord;
  readonly destination: Coord;
  readonly personnel: ArmyPersonnel;
}
