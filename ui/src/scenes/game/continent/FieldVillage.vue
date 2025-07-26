<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import type { PublicFieldImpl } from '@/core/model/continent/public-field';
import type { PublicVillageImpl } from '@/core/model/village/public-village';
import { Badge, HoverCard, HoverCardContent, HoverCardTrigger } from '@tb-dev/vue-components';

const props = defineProps<{
  field: PublicFieldImpl;
  village: PublicVillageImpl;
}>();

const color = computed(() => {
  switch (props.village.owner.kind) {
    case 'bot': {
      return 'bg-amber-950';
    }
    case 'player': {
      return 'bg-primary';
    }
    case 'precursor': {
      return getPrecursorColor(props.village.owner.id);
    }
    default:
      return 'bg-transparent';
  }
});

function getPrecursorColor(id: PrecursorId) {
  switch (id) {
    case 'A': {
      return 'bg-green-900';
    }
    case 'B': {
      return 'bg-purple-900';
    }
  }
}
</script>

<template>
  <HoverCard :open-delay="200" :close-delay="100">
    <HoverCardTrigger as-child>
      <div class="absolute inset-0 flex items-center justify-center overflow-hidden select-none">
        <div :class="color" class="size-[75%] cursor-pointer rounded-full"></div>
      </div>
    </HoverCardTrigger>

    <HoverCardContent>
      <div class="flex flex-col select-none">
        <div class="flex flex-col">
          <div class="flex justify-between">
            <h1 class="text-lg">{{ village.name }}</h1>
            <Badge variant="outline" size="sm">{{ field.id }}</Badge>
          </div>
          <h2 class="text-muted-foreground text-sm">
            <span v-if="village.owner.kind === 'player'">
              {{ village.owner.id }}
            </span>
            <span v-else-if="village.owner.kind === 'bot'">
              {{ `Bot ${village.owner.id}` }}
            </span>
            <span v-else-if="village.owner.kind === 'precursor'">
              {{ `Precursor ${village.owner.id}` }}
            </span>
          </h2>
        </div>
      </div>
    </HoverCardContent>
  </HoverCard>
</template>
