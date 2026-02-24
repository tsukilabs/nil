<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { useMutex } from '@tb-dev/vue';
import { useRouter } from 'vue-router';
import { useSettings } from '@/stores/settings';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import { computed, onBeforeMount, reactive } from 'vue';
import { isValidPassword, isValidPlayerId } from '@/lib/schema';
import ButtonSpinner from '@/components/button/ButtonSpinner.vue';
import { go, QUERY_SIGN_IN_USER, QUERY_SIGN_UP_USER } from '@/router';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle, Input, Label } from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const settings = useSettings();

interface User {
  name: Option<string>;
  password: Option<string>;
}

const user = reactive<User>({
  name: null,
  password: null,
});

const { locked, lock } = useMutex();
const canSignIn = computed(() => {
  return (
    isValidPlayerId(user.name) &&
    isValidPassword(user.password)
  );
});

onBeforeMount(() => {
  const url = new URL(window.location.href);
  user.name = url.searchParams.get(QUERY_SIGN_IN_USER);
});

async function signIn() {
  await lock(async () => {
    if (
      isValidPlayerId(user.name) &&
      isValidPassword(user.password)
    ) {
      const token = await commands.authorize(user.name, user.password);
      await commands.updateClient({
        serverAddr: { kind: 'remote' },
        playerId: user.name,
        playerPassword: user.password,
        authorizationToken: token,
      });

      settings.auth.token = token;
      await go('lobby');
    }
  });
}

async function goToSignUpScene() {
  await go('sign-up', { query: { [QUERY_SIGN_UP_USER]: user.name } });
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
            @keydown.enter="signIn"
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
            @keydown.enter="signIn"
          />
        </Label>
      </CardContent>

      <CardFooter class="grid grid-cols-3">
        <ButtonSpinner :loading="locked" :disabled="locked || !canSignIn" @click="signIn">
          {{ t('sign-in') }}
        </ButtonSpinner>

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
