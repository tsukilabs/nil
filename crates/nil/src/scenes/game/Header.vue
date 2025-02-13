<script setup lang="ts">
import { cn } from '@/lib/utils';
import { Game } from '@/core/game';
import { ButtonLink, SidebarTrigger } from '@/components';

type Props = {
  class?: string;
};

const props = defineProps<Props>();

const { village } = Game.use();
</script>

<template>
  <header :class="cn('flex items-center justify-between', props.class)">
    <div class="flex items-center gap-2">
      <SidebarTrigger />
      <ButtonLink to="village" variant="ghost" button-class="py-2 text-lg">
        <span>{{ village?.name }}</span>
        <span>({{ village?.coord.format() }})</span>
      </ButtonLink>
    </div>
    <div></div>
  </header>
</template>
