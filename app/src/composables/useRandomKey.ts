// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { readonly, ref } from "vue";

export function useRandomKey() {
  const key = ref<string>(crypto.randomUUID());

  function updateRandomKey() {
    key.value = crypto.randomUUID();
  }

  return {
    key: readonly(key),
    updateRandomKey,
  };
}
