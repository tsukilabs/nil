<script setup lang="ts"></script>

<template>
  <div class="absolute inset-0 overflow-hidden">
    <RouterView #default="{ Component }">
      <template v-if="Component">
        <component :is="Component" />
      </template>
    </RouterView>
  </div>
</template>
