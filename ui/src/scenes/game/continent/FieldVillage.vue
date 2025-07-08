<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import type { PublicVillageImpl } from '@/core/model/village/public';
import type { PublicFieldImpl } from '@/core/model/continent/public-field';
import { Badge, HoverCard, HoverCardContent, HoverCardTrigger } from '@tb-dev/vue-components';

defineProps<{
  field: PublicFieldImpl;
  village: PublicVillageImpl;
}>();
</script>

<template>
  <HoverCard :open-delay="200" :close-delay="100">
    <HoverCardTrigger as-child>
      <div class="absolute inset-0 flex items-center justify-center overflow-hidden select-none">
        <div class="bg-primary size-[75%] rounded-full"></div>
      </div>
    </HoverCardTrigger>

    <HoverCardContent>
      <div class="flex flex-col select-none">
        <div class="flex flex-col">
          <div class="flex justify-between">
            <h1 class="text-lg">{{ village.name }}</h1>
            <Badge variant="outline" size="sm">{{ field.id }}</Badge>
          </div>
          <h2 v-if="village.owner.kind === 'player'" class="text-muted-foreground text-sm">
            {{ village.owner.id }}
          </h2>
        </div>
      </div>
    </HoverCardContent>
  </HoverCard>
</template>
