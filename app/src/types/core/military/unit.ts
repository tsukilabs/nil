// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export type UnitId = AcademyUnitId | StableUnitId | WorkshopUnitId;

export type AcademyUnitId = 'archer' | 'axeman' | 'pikeman' | 'swordsman';

export type StableUnitId = 'heavy-cavalry' | 'light-cavalry';

export type WorkshopUnitId = 'ram';

export type UnitKind = 'infantry' | 'cavalry' | 'ranged';

export interface UnitStats {
  readonly attack: number;
  readonly infantryDefense: number;
  readonly cavalryDefense: number;
  readonly rangedDefense: number;
  readonly rangedDebuff: number;
  readonly speed: number;
  readonly haul: number;
}
