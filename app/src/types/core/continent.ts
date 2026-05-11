// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { ContinentIndex, Coord, PublicField } from "@tsukilabs/nil-bindings";

export type PublicFieldKind = PublicField["kind"];

export type CoordTuple = [Coord["x"], Coord["y"]];

export type ContinentKey = Coord | CoordTuple | ContinentIndex;
