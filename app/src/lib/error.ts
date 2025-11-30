// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as dialog from '@/lib/dialog';
import type { Option } from '@tb-dev/utils';

export function handleError(err: unknown, message?: Option<string>) {
  if (__DEBUG_ASSERTIONS__) {
    if (err instanceof Error) {
      console.error(`${err.message}\n${err.stack}`);
    }
    else {
      console.error(err);
    }
  }

  if (message) {
    dialog.error(message);
  }
  else if (err instanceof Error) {
    dialog.error(err.message);
  }
  else {
    dialog.error(String(err));
  }
}
