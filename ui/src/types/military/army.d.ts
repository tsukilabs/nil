// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Army {
  readonly personnel: ArmyPersonnel;
  readonly state: ArmyState;
  readonly owner: ArmyOwner;
}

interface ArmyPersonnel {
  readonly archer: Squad;
  readonly axeman: Squad;
  readonly heavyCavalry: Squad;
  readonly lightCavalry: Squad;
  readonly pikeman: Squad;
  readonly swordsman: Squad;
}

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
