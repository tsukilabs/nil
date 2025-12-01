// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Military {
  readonly continent: ReadonlyMap<number, readonly Army[]>;
  readonly continentSize: number;
  readonly maneuvers: ReadonlyMap<ManeuverId, readonly Maneuver[]>;
}
