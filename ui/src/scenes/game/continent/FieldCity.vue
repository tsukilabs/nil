<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import type { RouteLocationAsRelative } from 'vue-router';
import type { PublicCityImpl } from '@/core/model/city/public-city';
import type { PublicFieldImpl } from '@/core/model/continent/public-field';
import { Badge, HoverCard, HoverCardContent, HoverCardTrigger } from '@tb-dev/vue-components';

const props = defineProps<{
  field: PublicFieldImpl;
  city: PublicCityImpl;
}>();

const to = computed<RouteLocationAsRelative>(() => {
  const ckey = props.city.coord.toIndexString();
  return { name: 'profile-city' satisfies ProfileScene, params: { ckey } };
});

const color = computed(() => {
  switch (props.city.owner.kind) {
    case 'bot': {
      return 'bg-amber-950';
    }
    case 'player': {
      return 'bg-primary';
    }
    case 'precursor': {
      return getPrecursorColor(props.city.owner.id);
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
        <RouterLink :to :class="color" class="size-[75%] cursor-pointer rounded-full" />
      </div>
    </HoverCardTrigger>

    <HoverCardContent>
      <div class="flex flex-col select-none">
        <div class="flex justify-between">
          <div class="flex flex-col overflow-hidden">
            <h1 class="ellipsis text-lg">
              <RouterLink :to>{{ city.name }}</RouterLink>
            </h1>
            <h2 class="text-muted-foreground text-sm">
              <span v-if="city.owner.kind === 'player'">
                {{ city.owner.id }}
              </span>
              <span v-else-if="city.owner.kind === 'bot'">
                {{ `Bot ${city.owner.id}` }}
              </span>
              <span v-else-if="city.owner.kind === 'precursor'">
                {{ `Precursor ${city.owner.id}` }}
              </span>
            </h2>
          </div>

          <Badge variant="outline" size="sm" class="max-h-fit py-1">
            <RouterLink :to>{{ field.id }}</RouterLink>
          </Badge>
        </div>
      </div>
    </HoverCardContent>
  </HoverCard>
</template>
