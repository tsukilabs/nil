// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { useSettings } from '@/settings';
import { attemptAsync } from 'es-toolkit';
import { handleError } from '@/lib/error';
import { shallowRef, type ShallowRef } from 'vue';
import { check } from '@tauri-apps/plugin-updater';
import { openUrl } from '@tauri-apps/plugin-opener';
import { relaunch } from '@tauri-apps/plugin-process';

export interface Update {
  readonly version: string;
  readonly currentVersion: string;
  readonly install: () => Promise<void>;
  readonly openChangelog: () => Promise<void>;
}

export function useUpdate() {
  const update = shallowRef<Option<Update>>();
  if (__DESKTOP__) void checkForUpdates(update);
  return update;
}

async function checkForUpdates(updateRef: ShallowRef<Option<Update>>) {
  const [_, update] = await attemptAsync(check);
  if (update) {
    const install = async () => {
      updateRef.value = null;
      if (!__DEBUG_ASSERTIONS__) {
        try {
          await update.downloadAndInstall();
          await relaunch();
        }
        catch (err) {
          handleError(err);
        }
      }
    };

    const settings = useSettings();

    if (settings.autoUpdate) {
      void install();
    }
    else {
      updateRef.value = {
        version: update.version,
        currentVersion: update.currentVersion,
        install,
        openChangelog,
      };
    }
  }
}

async function openChangelog() {
  try {
    await openUrl('https://github.com/tsukilabs/nil/releases/latest');
  }
  catch (err) {
    handleError(err);
  }
}
