// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { sonner } from '@tb-dev/vue-components';
import type { ExternalToast } from '@tb-dev/vue-sonner';

type ToastOptions = Partial<ExternalToast>;

const defaultToastOptions: ToastOptions = {
  closeButton: false,
  dismissible: true,
  duration: 3000,
};

export const toast = {
  error(text: string, options: ToastOptions = {}) {
    sonner.error(text, { ...defaultToastOptions, ...options });
  },
  info(text: string, options: ToastOptions = {}) {
    sonner.info(text, { ...defaultToastOptions, ...options });
  },
  success(text: string, options: ToastOptions = {}) {
    sonner.success(text, { ...defaultToastOptions, ...options });
  },
  warning(text: string, options: ToastOptions = {}) {
    sonner.warning(text, { ...defaultToastOptions, ...options });
  },
} as const;
