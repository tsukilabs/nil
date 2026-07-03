// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toast } from "@/lib/toast";
import type { Option } from "@tb-dev/utils";

export function handleError(err: unknown, message?: Option<string>) {
  if (__DEBUG_ASSERTIONS__ && err) {
    if (Error.isError(err)) {
      console.error(`${err.message}\n${err.stack}`);
    }
    else {
      console.error(err);
    }
  }

  // eslint-disable-next-line @typescript-eslint/prefer-nullish-coalescing
  message ||= Error.isError(err) ? err.message : String(err);

  if (message) {
    toast.error(message);
  }
}
