// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Player {
  readonly id: PlayerId;
  readonly resources: Resources;
  readonly status: PlayerStatus;
}

type PlayerId = string;

type PlayerStatus = 'active' | 'guest' | 'inactive';

interface PlayerOptions {
  readonly id: PlayerId;
}
