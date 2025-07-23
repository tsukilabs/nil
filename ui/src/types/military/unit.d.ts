// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type UnitId = 'archer' | 'axeman' | 'heavy-cavalry' | 'light-cavalry' | 'pikeman' | 'swordsman';

type UnitKind = 'infantry' | 'cavalry' | 'ranged';

interface UnitStats {
  readonly attack: number;
  readonly infantryDefense: number;
  readonly cavalryDefense: number;
  readonly rangedDefense: number;
  readonly rangedDebuff: number;
  readonly speed: number;
  readonly haul: number;
}
