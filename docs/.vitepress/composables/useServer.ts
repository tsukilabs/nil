import { computed } from 'vue';
import type { Option } from '@tb-dev/utils';
import { useWorlds, type World } from './useWorlds';
import { useServerVersion } from './useServerVersion';

export interface NilServer {
  readonly version: string;
  readonly worlds: readonly World[];
}

export function useServer() {
  const { version } = useServerVersion();
  const { worlds } = useWorlds();

  return computed<Option<NilServer>>(() => {
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
