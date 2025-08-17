<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import HeaderMenu from './HeaderMenu.vue';
import Loading from '@/components/Loading.vue';
import { useAcademy } from '@/composables/infrastructure/useBuilding';
import { Card, CardContent, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n();

const academy = useAcademy();
</script>

<template>
  <div class="game-layout">
    <Card v-if="academy" class="size-full">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span>{{ `${t('academy')} (${t('level-x', [academy.level])})` }}</span>
            <HeaderMenu />
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full overflow-auto px-2 py-0">
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
