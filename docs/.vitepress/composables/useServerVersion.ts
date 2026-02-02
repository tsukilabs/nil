import { get } from '../lib/api';
import { asyncRef } from '@tb-dev/vue';

export function useServerVersion() {
  const { state, isLoading } = asyncRef(null, async () => {
    return get('version').then((it) => it.text());
  });

  return {
    version: state as Readonly<typeof state>,
    loading: isLoading,
  };
}
