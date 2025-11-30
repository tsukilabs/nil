// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface PublicPrecursor {
  readonly id: PrecursorId;
  readonly origin: Coord;
}

type PrecursorId = 'A' | 'B';
