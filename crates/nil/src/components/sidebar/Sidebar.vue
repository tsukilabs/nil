<script setup lang="ts">
import type { VNode } from 'vue';
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarInset,
  SidebarProvider,
} from '../base/sidebar';

type Props = {
  defaultOpen?: boolean;
};

defineProps<Props>();

defineSlots<{
  content?: () => VNode;
  footer?: () => VNode;
  header?: () => VNode;
  inset: () => VNode;
}>();
</script>

<template>
  <SidebarProvider :default-open>
    <Sidebar>
      <SidebarHeader v-if="$slots.header">
        <slot name="header"></slot>
      </SidebarHeader>
      <SidebarContent v-if="$slots.content">
        <slot name="content"></slot>
      </SidebarContent>
      <SidebarFooter v-if="$slots.footer">
        <slot name="footer"></slot>
      </SidebarFooter>
    </Sidebar>
    <SidebarInset>
      <slot name="inset"></slot>
    </SidebarInset>
  </SidebarProvider>
</template>
