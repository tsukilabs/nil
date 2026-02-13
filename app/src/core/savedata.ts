// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { leaveGame } from './game';
import * as commands from '@/commands';
import { fromZoned } from '@/lib/date';
import { Semaphore } from 'es-toolkit';
import { go, QUERY_LOAD_LOCAL_GAME_PATH } from '@/router';
import { compareDesc as compareDateDesc } from 'date-fns';
import { appDataDir, basename, extname, join } from '@tauri-apps/api/path';
import { exists, lstat, mkdir, readDir, remove as removeFile } from '@tauri-apps/plugin-fs';

export class SavedataFile {
  private constructor(
    public readonly path: string,
    public readonly name: string,
    public readonly date: Date,
    public readonly info: SavedataInfo,
  ) {}

  public async load() {
    await commands.allowScope(this.path);
    await leaveGame({ navigate: false });
    await go('load-local-game', {
      query: {
        [QUERY_LOAD_LOCAL_GAME_PATH]: this.path,
      },
    });
  }

  public async remove() {
    await removeFile(this.path);
  }

  public static async read(path: string) {
    const name = await basename(path, '.nil');
    const info = await commands.readSavedataInfo(path);
    const date = fromZoned(info.savedAt);

    return new SavedataFile(path, name, date, info);
  }
}

export interface SavedataInfo {
  readonly worldName: string;
  readonly round: RoundId;
  readonly version: string;
  readonly savedAt: string;
}

export async function getSavedataFiles() {
  const files: SavedataFile[] = [];
  const dir = await savedataDir();

  if (!(await exists(dir))) {
    await mkdir(dir, { recursive: true });
  }

  const entries = await readDir(dir);
  const semaphore = new Semaphore(10);

  if (entries.length > 0) {
    await Promise.all(entries.map(async (entry) => {
      await semaphore.acquire();
      try {
        if (entry.isFile && await extname(entry.name) === 'nil') {
          const path = await join(dir, entry.name);
          files.push(await SavedataFile.read(path));
        }
      }
      catch (err) {
        if (__DEBUG_ASSERTIONS__) {
          console.error(err);
        }
      }
      finally {
        semaphore.release();
      }
    }));

    files.sort((a, b) => compareDateDesc(a.date, b.date));
  }

  return files;
}

export async function isSavedataFile(path: string) {
  if (await commands.exists(path)) {
    const metadata = await lstat(path);
    if (metadata.isFile) {
      const extension = await extname(path);
      return extension.toLowerCase() === 'nil';
    }
  }

  return false;
}

export async function loadSavedataFromArgs() {
  if (__DESKTOP__ && !__DEBUG_ASSERTIONS__) {
    const args = await commands.args();
    if (args[1] && await isSavedataFile(args[1])) {
      const savedata = await SavedataFile.read(args[1]);
      await savedata.load();
    }
  }
}

export async function saveLocalGame() {
  const path = await savedataDir();
  await commands.saveLocalWorld(path);
}

export async function savedataDir() {
  const dataDir = await appDataDir();
  return join(dataDir, 'savedata');
}
