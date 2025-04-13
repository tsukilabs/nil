<script setup lang="ts">
import { ButtonLink, cn, SidebarTrigger } from '@tb-dev/vue';

type Props = {
  class?: string;
};

const props = defineProps<Props>();

const { village } = NIL.village.refs();
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
