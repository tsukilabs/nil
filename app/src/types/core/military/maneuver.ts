// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Ruler } from '@/types/core/ruler';
import type { Coord } from '@/types/core/continent';
import type { Resources } from '@/types/core/resources';
import type { ArmyId, ArmyPersonnel } from '@/types/core/military/army';

export interface Maneuver {
  readonly id: ManeuverId;
  readonly origin: Coord;
  readonly destination: Coord;
  readonly army: ArmyId;
  readonly kind: ManeuverKind;
  readonly direction: ManeuverDirection;
  readonly state: ManeuverState;
  readonly speed: number;
  readonly hauledResources: Option<ManeuverHaul>;
}

export type ManeuverId = string;

export type ManeuverKind = 'attack' | 'support';

export type ManeuverDirection = 'going' | 'returning';

export type ManeuverDistance = number;

export type ManeuverState = ManeuverStateDone | ManeuverStatePending;

export interface ManeuverStateDone {
  readonly kind: 'done';
}

export interface ManeuverStatePending {
  readonly kind: 'pending';
  readonly distance: ManeuverDistance;
}

export interface ManeuverHaul {
  readonly ruler: Ruler;
  readonly resources: Resources;
}

export interface ManeuverRequest {
  readonly kind: ManeuverKind;
  readonly origin: Coord;
  readonly destination: Coord;
  readonly personnel: ArmyPersonnel;
}
