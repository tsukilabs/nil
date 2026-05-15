// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { computed } from "vue";
import { useWorlds } from "./useWorlds";
import type { Option } from "@tb-dev/utils";
import { useServerVersion } from "./useServerVersion";
import type { RemoteWorld } from "@tsukilabs/nil-bindings";

export interface Server {
  readonly version: string;
  readonly worlds: readonly RemoteWorld[];
}

export function useServer() {
  const { version } = useServerVersion();
  const { worlds } = useWorlds();

  return computed<Option<Server>>(() => {
    if (version.value && worlds.value.length > 0) {
      return {
        version: version.value,
        worlds: worlds.value,
      };
    }
    else {
      return null;
    }
  });
}
