// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toast } from '@/lib/toast';
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
    toast.error(message);
  }
  else if (err instanceof Error) {
    toast.error(err.message);
  }
  else {
    toast.error(String(err));
  }
}
