<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import { computed, ref } from 'vue';
import { exitGame } from '@/core/game';
import { handleError } from '@/lib/error';
import { useUserStore } from '@/stores/user';
import { useBreakpoints } from '@tb-dev/vue';
import enUS from '@/locale/en-US/scenes/home.json';
import ptBR from '@/locale/pt-BR/scenes/home.json';
import { useUpdate } from '@/composables/useUpdate';
import LoadingButton from '@/components/LoadingButton.vue';
import { Alert, AlertDescription, AlertTitle, Button } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const update = useUpdate();
const { sm, md } = useBreakpoints();

const userStore = useUserStore();

const isLoadingOnlineScene = ref(false);
const disabled = computed(() => isLoadingOnlineScene.value);

async function goToOnlineScene() {
  try {
    isLoadingOnlineScene.value = true;
    await userStore.updateClient();
    if (await userStore.isAuthorizationTokenValid()) {
      await go('lobby');
    }
    else {
      await go('sign-in');
    }
  }
  catch (err) {
    handleError(err);
  }
  finally {
    isLoadingOnlineScene.value = false;
  }
}
</script>

<template>
  <div class="flex size-full flex-col items-center justify-center gap-2">
    <h1 class="font-syne-mono -mt-16 mb-8 text-5xl font-extrabold sm:text-6xl md:text-7xl">
      <span>Call of Nil</span>
    </h1>

    <div class="flex flex-col gap-2">
      <Button
        variant="default"
        :size="sm ? 'default' : 'lg'"
        :disabled
        role="link"
        tabindex="0"
        @click="() => go('host-local-game')"
      >
        <span>{{ t('host-game') }}</span>
      </Button>

      <Button
        variant="secondary"
        :size="sm ? 'default' : 'lg'"
        :disabled
        role="link"
        tabindex="0"
        @click="() => go('join-local-game')"
      >
        <span>{{ t('join-game') }}</span>
      </Button>

      <LoadingButton
        variant="secondary"
        :size="sm ? 'default' : 'lg'"
        :disabled
        :loading="isLoadingOnlineScene"
        role="link"
        tabindex="0"
        @click="goToOnlineScene"
      >
        <span>{{ t('online') }}</span>
      </LoadingButton>

      <Button
        variant="secondary"
        :size="sm ? 'default' : 'lg'"
        :disabled
        role="link"
        tabindex="0"
        @click="() => go('settings')"
      >
        <span>{{ t('settings') }}</span>
      </Button>

      <Button
        variant="secondary"
        :size="sm ? 'default' : 'lg'"
        :disabled
        @click="() => exitGame()"
      >
        <span>{{ t('exit') }}</span>
      </Button>
    </div>

    <Alert v-if="update && md" class="w-max fixed bottom-safe-4 right-safe-4 py-4">
      <AlertTitle>{{ t('update-available') }}</AlertTitle>
      <AlertDescription class="gap-2">
        <span>{{ t('version-ready', [update.version]) }}</span>
        <div class="grid grid-cols-2 items-center gap-2 justify-self-end">
          <Button variant="default" size="sm" @click="() => update?.install()">
            <span>{{ t('update') }}</span>
          </Button>

          <Button variant="secondary" size="sm" @click="() => update?.openChangelog()">
            <span>{{ t('whats-new') }}</span>
          </Button>
        </div>
      </AlertDescription>
    </Alert>
  </div>
</template>
