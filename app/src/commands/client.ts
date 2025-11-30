// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import { SocketAddrV4 } from '@/lib/net/addr-v4';

export async function startClient(id: PlayerId, addr: SocketAddrV4) {
  const serverAddr = addr.format();
  await invoke('start_client', { playerId: id, serverAddr });
}

export async function stopClient() {
  await invoke('stop_client');
}
