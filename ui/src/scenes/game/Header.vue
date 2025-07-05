<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Round from './Round.vue';
import type { MaybePromise } from '@tb-dev/utils';
import { ButtonLink, SidebarTrigger } from '@tb-dev/vue-components';

defineProps<{ onTurnEnd: () => MaybePromise<void> }>();

const { village } = NIL.village.refs();
</script>

<template>
  <header class="flex items-center justify-between">
    <div class="flex items-center gap-2">
      <SidebarTrigger />
      <ButtonLink to="village" variant="ghost" button-class="py-2 text-base lg:text-lg">
        <span>{{ village?.name }}</span>
        <span>({{ village?.coord.format() }})</span>
      </ButtonLink>
    </div>

    <div class="flex items-center">
      <Round @turn-end="onTurnEnd" />
    </div>
  </header>
</template>
