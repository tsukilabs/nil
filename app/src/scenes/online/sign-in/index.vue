<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import type { User } from './types';
import * as commands from '@/commands';
import { useMutex } from '@tb-dev/vue';
import { useRouter } from 'vue-router';
import { useUserStore } from '@/stores/user';
import { computed, onBeforeMount, ref } from 'vue';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import { isValidPassword, isValidPlayerId } from '@/lib/schema';
import { QUERY_SIGN_IN_USER, QUERY_SIGN_UP_USER } from '@/router/online';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle, Input, Label } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const userStore = useUserStore();

const user = ref<User>({
  name: null,
  password: null,
});

const { locked, lock } = useMutex();
const canSignIn = computed(() => {
  return (
    isValidPlayerId(user.value.name) &&
    isValidPassword(user.value.password)
  );
});

onBeforeMount(() => {
  const url = new URL(window.location.href);
  user.value.name = url.searchParams.get(QUERY_SIGN_IN_USER);
});

async function signIn() {
  await lock(async () => {
    if (
      isValidPlayerId(user.value.name) &&
      isValidPassword(user.value.password)
    ) {
      const token = await commands.authorize(user.value.name, user.value.password);
      await commands.updateClient({
        serverAddr: { kind: 'remote' },
        playerId: user.value.name,
        playerPassword: user.value.password,
        authorizationToken: token,
      });

      userStore.authorizationToken = token;
      await go('lobby');
    }
  });
}

async function goToSignUpScene() {
  await go('sign-up', { query: { [QUERY_SIGN_UP_USER]: user.value.name } });
}
</script>

<template>
  <div class="card-layout">
    <Card>
      <CardHeader>
        <CardTitle>{{ t('sign-in') }}</CardTitle>
      </CardHeader>

      <CardContent>
        <Label>
          <span>{{ t('user') }}</span>
          <Input
            v-model.trim="user.name"
            type="text"
            :disabled="locked"
            :minlength="1"
            :maxlength="20"
          />
        </Label>
        <Label>
          <span>{{ t('password') }}</span>
          <Input
            v-model="user.password"
            type="password"
            :disabled="locked"
            :minlength="3"
            :maxlength="50"
          />
        </Label>
      </CardContent>

      <CardFooter class="grid grid-cols-3">
        <Button :disabled="locked || !canSignIn" @click="signIn">
          {{ t('sign-in') }}
        </Button>

        <Button
          variant="secondary"
          :disabled="locked"
          role="link"
          tabindex="0"
          @click="goToSignUpScene"
        >
          <span>{{ t('sign-up') }}</span>
        </Button>

        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
