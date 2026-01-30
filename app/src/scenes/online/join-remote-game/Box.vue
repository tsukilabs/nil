<script setup lang="ts">
defineProps<{
  label: string;
  content: number | string;
}>();
</script>

<template>
  <div class="flex h-16 flex-col items-center justify-center">
    <span class="text-muted-foreground text-xs md:text-sm break-all wrap-anywhere">{{ label }}</span>
    <span class="text-sm md:text-base font-semibold break-all wrap-anywhere">{{ content }}</span>
  </div>
</template>
