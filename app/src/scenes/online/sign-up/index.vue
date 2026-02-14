<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { go } from '@/router';
import { useI18n } from 'vue-i18n';
import * as commands from '@/commands';
import { useMutex } from '@tb-dev/vue';
import { useRouter } from 'vue-router';
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

interface NewUser {
  name: Option<PlayerId>;
  password: Option<string>;
  password2: Option<string>;
}

const newUser = ref<NewUser>({
  name: null,
  password: null,
  password2: null,
});

const { locked, lock } = useMutex();
const canCreate = computed(() => {
  return (
    isValidPlayerId(newUser.value.name) &&
    isValidPassword(newUser.value.password) &&
    newUser.value.password === newUser.value.password2
  );
});

onBeforeMount(() => {
  const url = new URL(window.location.href);
  newUser.value.name = url.searchParams.get(QUERY_SIGN_UP_USER);
});

async function signUp() {
  await lock(async () => {
    if (
      isValidPlayerId(newUser.value.name) &&
      isValidPassword(newUser.value.password) &&
      newUser.value.password === newUser.value.password2
    ) {
      await commands.createUser(newUser.value.name, newUser.value.password);
      await go('sign-in', { query: { [QUERY_SIGN_IN_USER]: newUser.value.name } });
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
        <Button :disabled="locked || !canCreate" @click="signUp">
          {{ t('create') }}
        </Button>
        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
