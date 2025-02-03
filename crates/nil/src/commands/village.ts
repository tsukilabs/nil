import type { Coord } from '@/types/world';
import { invoke } from '@tauri-apps/api/core';
import type { Village } from '@/types/village';

export async function getVillage(coord: Coord): Promise<Village> {
  return invoke('get_village', { coord });
}
