// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Military {
  readonly continent: ReadonlyMap<ContinentIndex, readonly Army[]>;
  readonly continentSize: ContinentSize;
  readonly maneuvers: ReadonlyMap<ManeuverId, Maneuver>;
}
