// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PlayerId } from '@/types/core/player';

export interface CreateUserRequest {
  readonly player: PlayerId;
  readonly password: string;
}

export interface UserExistsRequest {
  readonly user: PlayerId;
}
