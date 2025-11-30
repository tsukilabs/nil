// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { SocketAddrV4 } from '@/lib/net/addr-v4';

export function startClient(id: PlayerId, addr: SocketAddrV4) {
  const serverAddr = addr.format();
  return invoke<null>('start_client', { playerId: id, serverAddr });
}

export function stopClient() {
  return invoke<null>('stop_client');
}
