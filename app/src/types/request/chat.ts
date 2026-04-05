// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/core/world';

export interface GetChatHistoryRequest {
  readonly world: WorldId;
}

export interface PushChatMessageRequest {
  readonly world: WorldId;
  readonly message: string;
}
