import { invoke } from '@tauri-apps/api/core';
import type { SocketAddrV4 } from '@/lib/net/addr';

export function startClient(addr: SocketAddrV4) {
  const serverAddr = addr.format();
  return invoke<null>('start_client', { serverAddr });
}

export function stopClient() {
  return invoke<null>('stop_client');
}
