// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Maneuver {
  readonly id: ManeuverId;
  readonly direction: ManeuverDirection;
  readonly origin: Coord;
  readonly army: ArmyId;
  readonly kind: ManeuverKind;
  readonly destination: Coord;
  readonly state: ManeuverState;
  readonly hauledResources: Option<ManeuverHaul>;
}

type ManeuverId = string;

type ManeuverKind = 'attack' | 'support';

type ManeuverDirection = 'going' | 'returning';

type ManeuverDistance = number;

type ManeuverState = ManeuverStateDone | ManeuverStatePending;

interface ManeuverStateDone {
  readonly kind: 'done';
}

interface ManeuverStatePending {
  readonly kind: 'pending';
  readonly distance: ManeuverDistance;
}

interface ManeuverHaul {
  readonly ruler: Ruler;
  readonly resources: Resources;
}

interface ManeuverRequest {
  readonly kind: ManeuverKind;
  readonly origin: Coord;
  readonly destination: Coord;
  readonly personnel: ArmyPersonnel;
}
