import { invoke } from '@tauri-apps/api/core';
import type { WorldConfig } from '@/types/world';
import type { ServerInfo } from '@/types/server';

export function getServerVersion() {
  return invoke<string>('get_server_version');
}

export function isServerReady() {
  return invoke<boolean>('is_server_ready');
}

export function startServer(worldConfig: WorldConfig) {
  return invoke<ServerInfo>('start_server', { worldConfig });
}

export function stopServer() {
  return invoke<null>('stop_server');
}
