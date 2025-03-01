import { invoke } from '@tauri-apps/api/core';
import { SocketAddrV4 } from '@/lib/net/addr-v4';

export * from './round';
export * from './world';
export * from './player';
export * from './village';

export async function getServerAddr() {
  const addr = await invoke<string>('get_server_addr');
  return SocketAddrV4.parse(addr);
}

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

export async function startServer(worldOptions: WorldOptions) {
  const addr = await invoke<string>('start_server', { worldOptions });
  return SocketAddrV4.parse(addr);
}

export function stopClient() {
  return invoke<null>('stop_client');
}

export function stopServer() {
  return invoke<null>('stop_server');
}
