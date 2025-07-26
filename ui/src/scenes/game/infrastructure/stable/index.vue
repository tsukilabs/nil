<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useStable } from '@/composables/infrastructure/useBuilding';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Loading,
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from '@tb-dev/vue-components';

const { t } = useI18n();

const stable = useStable();
</script>

<template>
  <div class="game-layout">
    <Card v-if="stable" class="size-full">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span>{{ `${t('stable')} (${t('level-x', [stable.level])})` }}</span>

            <NavigationMenu>
              <NavigationMenuList>
                <NavigationMenuItem>
                  <NavigationMenuLink as-child :class="navigationMenuTriggerStyle()">
                    <RouterLink :to="{ name: 'stable' satisfies StableScene }">
                      {{ t('recruitment') }}
                    </RouterLink>
                  </NavigationMenuLink>
                </NavigationMenuItem>

                <NavigationMenuItem>
                  <NavigationMenuLink as-child :class="navigationMenuTriggerStyle()">
                    <RouterLink :to="{ name: 'stable-settings' satisfies StableScene }">
                      {{ t('settings') }}
                    </RouterLink>
                  </NavigationMenuLink>
                </NavigationMenuItem>
              </NavigationMenuList>
            </NavigationMenu>
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="overflow-x-hidden overflow-y-auto px-2 py-0">
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
