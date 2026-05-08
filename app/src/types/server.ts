// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { WorldId } from '@/types/bindings';

export interface LocalServer {
  readonly worldId: WorldId;
  readonly addr: string;
}

export type ServerAddr = ServerAddrLocal | ServerAddrRemote;

export interface ServerAddrLocal {
  readonly kind: 'local';
  readonly addr: string;
}

export interface ServerAddrRemote {
  readonly kind: 'remote';
}
