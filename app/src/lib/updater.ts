import { attemptAsync } from 'es-toolkit';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export async function checkForUpdates() {
  if (__DESKTOP__ && !__DEBUG_ASSERTIONS__) {
    const [_, update] = await attemptAsync(check);
    if (update) {
      await update.downloadAndInstall();
      await relaunch();
    }
  }
}
