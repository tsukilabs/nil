<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import HeaderMenu from './HeaderMenu.vue';
import Loading from '@/components/Loading.vue';
import { usePrefecture } from '@/composables/infrastructure/useBuilding';
import enUS from '@/locale/en-US/scenes/game/infrastructure/prefecture.json';
import ptBR from '@/locale/pt-BR/scenes/game/infrastructure/prefecture.json';
import { Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const prefecture = usePrefecture();
</script>

<template>
  <div class="game-layout">
    <Card v-if="prefecture" class="size-full">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span>{{ `${t('prefecture')} (${t('level-x', [prefecture.level])})` }}</span>
            <HeaderMenu />
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="overflow-auto px-2 py-0">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <KeepAlive>
              <Suspense>
                <component :is="Component" />
                <template #fallback>
                  <Loading class="absolute inset-0" />
                </template>
              </Suspense>
            </KeepAlive>
          </template>
        </RouterView>
      </CardContent>
    </Card>
  </div>
</template>
