// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Army {
  readonly id: ArmyId;
  readonly personnel: ArmyPersonnel;
  readonly owner: ArmyOwner;
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

type ArmyState = 'idle';

type ArmyOwner = ArmyOwnerBot | ArmyOwnerPlayer | ArmyOwnerPrecursor;

interface ArmyOwnerBot {
  readonly kind: 'bot';
  readonly id: BotId;
}

interface ArmyOwnerPlayer {
  readonly kind: 'player';
  readonly id: PlayerId;
}

interface ArmyOwnerPrecursor {
  readonly kind: 'precursor';
  readonly id: PrecursorId;
}

type WritableArmyPersonnel = {
  [unit in keyof Writable<ArmyPersonnel>]: Writable<Squad>;
};

type ArmyPersonnelSize = {
  [unit in keyof ArmyPersonnel]: number;
};
