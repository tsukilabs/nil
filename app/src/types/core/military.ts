// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { ArmyPersonnel, Squad } from '@/types/bindings';

export interface ArmyAcademyPersonnel {
  readonly archer: Squad;
  readonly axeman: Squad;
  readonly pikeman: Squad;
  readonly swordsman: Squad;
}

export interface ArmyStablePersonnel {
  readonly heavyCavalry: Squad;
  readonly lightCavalry: Squad;
}

export interface ArmyWorkshopPersonnel {
  readonly ram: Squad;
}

export type ArmyPersonnelSize = {
  [unit in keyof ArmyPersonnel]: number;
};

export type SquadTuple = [Squad['unit'], Squad['size']];
