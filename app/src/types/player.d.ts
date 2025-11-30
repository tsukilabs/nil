// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface PublicPlayer {
  readonly id: PlayerId;
  readonly status: PlayerStatus;
}

interface Player extends PublicPlayer {
  readonly resources: Resources;
}

type PlayerId = string;

type PlayerStatus = 'active' | 'inactive';

interface PlayerOptions {
  readonly id: PlayerId;
}
