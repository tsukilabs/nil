<script setup lang="ts">
import { cn } from '@/lib/utils';
import { Button } from '../button';
import type { Component, VNode } from 'vue';
import type { Route } from '@/router/types';
import { ButtonLink } from '../button-link';

interface Props {
  buttonClass?: string;
  class?: string;
  icon: Component;
  iconClass?: string;
  iconSize?: number;
  to?: Route;
}

const props = defineProps<Props>();

defineSlots<{ default?: () => VNode }>();
</script>

<template>
  <ButtonLink v-if="to" :to variant="ghost" size="icon" :button-class>
    <component :is="icon" :class="iconClass" :size="iconSize" />
  </ButtonLink>
  <Button v-else variant="ghost" size="icon" :class="cn(props.class, buttonClass)">
    <component :is="icon" :class="iconClass" :size="iconSize" />
  </Button>
</template>
