// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Army {
  readonly id: ArmyId;
  readonly personnel: ArmyPersonnel;
  readonly owner: Ruler;
  readonly state: ArmyState;
}

interface ArmyPersonnel extends ArmyAcademyPersonnel, ArmyStablePersonnel {}

interface ArmyAcademyPersonnel {
  readonly archer: Squad;
  readonly axeman: Squad;
  readonly pikeman: Squad;
  readonly swordsman: Squad;
}

interface ArmyStablePersonnel {
  readonly heavyCavalry: Squad;
  readonly lightCavalry: Squad;
}

type ArmyId = string;

type ArmyState = ArmyStateIdle | ArmyStateManeuvering;

interface ArmyStateIdle {
  readonly kind: 'idle';
}

interface ArmyStateManeuvering {
  readonly kind: 'maneuvering';
  readonly maneuver: ManeuverId;
}

type WritableArmyPersonnel = {
  [unit in keyof Writable<ArmyPersonnel>]: Writable<Squad>;
};

type ArmyPersonnelSize = {
  [unit in keyof ArmyPersonnel]: number;
};
