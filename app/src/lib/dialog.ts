// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { sonner } from '@tb-dev/vue-components';
import { message } from '@tauri-apps/plugin-dialog';
import type { ExternalToast } from '@tb-dev/vue-sonner';

const defaultToastOptions: ExternalToast = {
  closeButton: false,
  dismissible: true,
  duration: 3000,
};

export function error(text: string) {
  if (__MOBILE__) {
    sonner.error(text, defaultToastOptions);
  }
  else {
    message(text, { kind: 'error', title: 'Error' }).catch(console.error);
  }
}

export function info(text: string) {
  if (__MOBILE__) {
    sonner.info(text, defaultToastOptions);
  }
  else {
    message(text, { kind: 'info', title: 'Info' }).catch(console.error);
  }
}

export function warn(text: string) {
  if (__MOBILE__) {
    sonner.warning(text, defaultToastOptions);
  }
  else {
    message(text, { kind: 'warning', title: 'Warning' }).catch(console.error);
  }
}
