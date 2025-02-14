import { invoke } from '@tauri-apps/api/core';
import type { ServerInfo } from '@/types/server';
import type { WorldOptions } from '@/types/world';
import type { SocketAddrV4 } from '@/lib/net/addr-v4';

export * from './round';
export * from './player';
export * from './village';

export function getServerVersion() {
  return invoke<string>('get_server_version');
}

export function isDev() {
  return invoke<boolean>('is_dev');
}

export function isServerReady() {
  return invoke<boolean>('is_server_ready');
}

export function showWindow() {
  return invoke<null>('show_window');
}

export function startClient(addr: SocketAddrV4) {
  const serverAddr = addr.format();
  return invoke<null>('start_client', { serverAddr });
}

export function startServer(worldOptions: WorldOptions) {
  return invoke<ServerInfo>('start_server', { worldOptions });
}

export function stopClient() {
  return invoke<null>('stop_client');
}

export function stopServer() {
  return invoke<null>('stop_server');
}
