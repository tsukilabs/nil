import { invoke } from '@tauri-apps/api/core';

export async function getVillage(coord: Coord): Promise<Village> {
  return invoke('get_village', { coord });
}
