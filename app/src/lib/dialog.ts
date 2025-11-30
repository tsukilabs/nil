// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { message, open, save } from '@tauri-apps/plugin-dialog';

export { message, open, save };

export function error(text: string) {
  message(text, { kind: 'error', title: 'Error' }).catch(console.error);
}

export function info(text: string) {
  message(text, { kind: 'info', title: 'Info' }).catch(console.error);
}

export function warn(text: string) {
  message(text, { kind: 'warning', title: 'Warning' }).catch(console.error);
}
