// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { formatDate, parse } from 'date-fns';
import { readDir } from '@tauri-apps/plugin-fs';
import { appDataDir, extname, join } from '@tauri-apps/api/path';

export interface SavedataFile {
  readonly name: string;
  readonly path: string;
  readonly date: Date;
}

const REGEX = /^(.+?)-(\d+?)\.nil$/i;
const DATE_FORMAT = 'yyyyMMddHHmmss';

export async function savedataDir() {
  const dataDir = await appDataDir();
  return join(dataDir, 'savedata');
}

export async function getSavedataFiles() {
  const files: SavedataFile[] = [];
  const dir = await savedataDir();
  const entries = await readDir(dir);

  const referenceDate = new Date();
  await Promise.all(entries.map(async (entry) => {
    if (entry.isFile && await extname(entry.name) === 'nil') {
      const matches = REGEX.exec(entry.name);
      if (matches?.[1] && matches[2]) {
        const name = matches[1];
        const path = await join(dir, entry.name);
        const date = parse(matches[2], DATE_FORMAT, referenceDate);
        files.push({ name, path, date });
      }
    }
  }));

  return files;
}

export async function saveGame() {
  const world = NIL.world.refs().config.value?.name;
  if (world) {
    const dir = await savedataDir();
    const date = formatDate(Date.now(), DATE_FORMAT);
    const path = await join(dir, `${world}-${date}.nil`);
    await commands.saveWorld(path);
  }
}
