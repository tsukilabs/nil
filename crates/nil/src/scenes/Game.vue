<script setup lang="ts"></script>

<template>
  <div class="absolute inset-0 overflow-hidden">
    <RouterView #default="{ Component }">
      <template v-if="Component">
        <Suspense>
          <component :is="Component" />
          <template #fallback>
            <div class="size-full"></div>
          </template>
        </Suspense>
      </template>
    </RouterView>
  </div>
</template>
