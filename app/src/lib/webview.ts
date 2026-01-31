// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getCurrentWebview } from '@tauri-apps/api/webview';

export async function clearAllBrowsingData() {
  if (__DESKTOP__) {
    const webview = getCurrentWebview();
    await webview.clearAllBrowsingData();
  }
}
