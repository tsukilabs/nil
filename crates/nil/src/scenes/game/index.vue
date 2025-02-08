<script setup lang="ts">
import Header from './Header.vue';
import { Game } from '@/core/game';
import { LogOut } from 'lucide-vue-next';
import { ButtonIcon, Sidebar } from '@/components';

const game = Game.use();
</script>

<template>
  <Sidebar>
    <template #inset>
      <div class="bg-muted/40 absolute inset-0 overflow-hidden">
        <Header class="bg-background absolute inset-x-0 top-0 h-16 border-b px-4" />
        <div class="absolute inset-x-0 bottom-0 top-16 overflow-hidden p-4">
          <RouterView #default="{ Component }">
            <template v-if="Component">
              <component :is="Component" />
            </template>
          </RouterView>
        </div>
      </div>
    </template>

    <template #content> Content </template>

    <template #footer>
      <div class="flex items-center justify-center gap-4">
        <ButtonIcon :icon="LogOut" button-class="p-4" @click="() => game.leave()" />
      </div>
    </template>
  </Sidebar>
</template>
