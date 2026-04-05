// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import type { Squad } from '@/types/core/military/squad';
import type { ManeuverId } from '@/types/core/military/maneuver';

export interface Army {
  readonly id: ArmyId;
  readonly personnel: ArmyPersonnel;
  readonly owner: Ruler;
  readonly state: ArmyState;
}

export interface ArmyPersonnel
  extends ArmyAcademyPersonnel, ArmyStablePersonnel, ArmyWorkshopPersonnel
{}

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

export type ArmyId = string;

export type ArmyState = ArmyStateIdle | ArmyStateManeuvering;

export interface ArmyStateIdle {
  readonly kind: 'idle';
}

export interface ArmyStateManeuvering {
  readonly kind: 'maneuvering';
  readonly maneuver: ManeuverId;
}

export type WritableArmyPersonnel = {
  [unit in keyof Writable<ArmyPersonnel>]: Writable<Squad>;
};

export type ArmyPersonnelSize = {
  [unit in keyof ArmyPersonnel]: number;
};
