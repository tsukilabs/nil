<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Button } from '@tb-dev/vue-components';
import { MessageSquareTextIcon } from 'lucide-vue-next';

defineProps<{ hasUnread: boolean; }>();
</script>

<template>
  <div class="relative">
    <Button variant="ghost" size="icon">
      <MessageSquareTextIcon />
    </Button>

    <div
      v-if="hasUnread"
      class="absolute top-[4px] right-[4px] size-[10px] min-h-[10px] min-w-[10px] overflow-hidden rounded-full bg-red-500"
    >
    </div>
  </div>
</template>
