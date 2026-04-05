// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PlayerId } from '@/types/core/player';

export interface AuthorizeRequest {
  readonly player: PlayerId;
  readonly password?: Option<string>;
}

export interface ValidateTokenRequest {
  readonly token: string;
}
