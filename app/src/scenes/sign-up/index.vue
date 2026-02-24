<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { useMutex } from '@tb-dev/vue';
import { useRouter } from 'vue-router';
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

interface NewUser {
  name: Option<PlayerId>;
  password: Option<string>;
  password2: Option<string>;
}

const newUser = reactive<NewUser>({
  name: null,
  password: null,
  password2: null,
});

const { locked, lock } = useMutex();
const canCreate = computed(() => {
  return (
    isValidPlayerId(newUser.name) &&
    isValidPassword(newUser.password) &&
    newUser.password === newUser.password2
  );
});

onBeforeMount(() => {
  const url = new URL(window.location.href);
  newUser.name = url.searchParams.get(QUERY_SIGN_UP_USER);
});

async function signUp() {
  await lock(async () => {
    if (
      isValidPlayerId(newUser.name) &&
      isValidPassword(newUser.password) &&
      newUser.password === newUser.password2
    ) {
      await commands.createUser(newUser.name, newUser.password);
      await go('sign-in', { query: { [QUERY_SIGN_IN_USER]: newUser.name } });
    }
  });
}
</script>

<template>
  <div class="card-layout">
    <Card>
      <CardHeader>
        <CardTitle>{{ t('sign-up') }}</CardTitle>
      </CardHeader>

      <CardContent>
        <Label>
          <span>{{ t('user') }}</span>
          <Input
            v-model.trim="newUser.name"
            type="text"
            :disabled="locked"
            :minlength="1"
            :maxlength="20"
            @keydown.enter="signUp"
          />
        </Label>
        <Label>
          <span>{{ t('password') }}</span>
          <Input
            v-model="newUser.password"
            type="password"
            :disabled="locked"
            :minlength="3"
            :maxlength="50"
            @keydown.enter="signUp"
          />
        </Label>
        <Label>
          <span>{{ t('confirm-password') }}</span>
          <Input
            v-model="newUser.password2"
            type="password"
            :disabled="locked"
            :minlength="3"
            :maxlength="50"
            @keydown.enter="signUp"
          />
        </Label>
      </CardContent>

      <CardFooter class="grid grid-cols-2">
        <ButtonSpinner :loading="locked" :disabled="locked || !canCreate" @click="signUp">
          {{ t('create') }}
        </ButtonSpinner>
        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
