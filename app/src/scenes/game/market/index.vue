<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import HeaderMenu from "./HeaderMenu.vue";
import { useI18n } from "@tsukilabs/nil-i18n";
import Loading from "@/components/Loading.vue";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";

const { t } = useI18n();
</script>

<template>
  <div class="game-layout">
    <Card class="size-full">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span>{{ t("market") }}</span>
            <HeaderMenu />
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="size-full overflow-auto px-2 py-0">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <Suspense>
              <component :is="Component" />
              <template #fallback>
                <Loading class="absolute inset-0" />
              </template>
            </Suspense>
          </template>
        </RouterView>
      </CardContent>
    </Card>
  </div>
</template>
