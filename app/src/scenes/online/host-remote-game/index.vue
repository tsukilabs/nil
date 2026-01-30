<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';
import { hostRemoteGame } from '@/core/game';
import { useUserStore } from '@/stores/user';
import { useSettings } from '@/stores/settings';
import { localRef, useMutex } from '@tb-dev/vue';
import enUS from '@/locale/en-US/scenes/online.json';
import ptBR from '@/locale/pt-BR/scenes/online.json';
import type { WritablePartial } from '@tb-dev/utils';
import { isValidNullishPassword, isWorldOptions } from '@/lib/schema';
import {
  Button,
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
  Checkbox,
  Input,
  Label,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
  Textarea,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const settings = useSettings();

const userStore = useUserStore();
const { authorizationToken } = storeToRefs(userStore);

const worldOptions = localRef<WritablePartial<WorldOptions>>('host-remote-game:world', {
  name: null,
  size: 100,
  locale: settings.locale,
  allowCheats: false,
});

const worldPassword = ref<Option<string>>();
const description = ref<Option<string>>();

const { locked, lock } = useMutex();
const isValidWorld = computed(() => isWorldOptions(worldOptions.value));
const canHost = computed(() => {
  return (
    isValidWorld.value &&
    Boolean(authorizationToken.value) &&
    isValidNullishPassword(worldPassword.value)
  );
});

async function host() {
  worldOptions.value.locale ??= settings.locale;
  await lock(async () => {
    if (isWorldOptions(worldOptions.value) && authorizationToken.value) {
      await hostRemoteGame({
        worldOptions: worldOptions.value,
        worldPassword: worldPassword.value,
        worldDescription: description.value,
        authorizationToken: authorizationToken.value,
      });
    }
  });
}
</script>

<template>
  <div class="card-layout">
    <Card>
      <CardHeader>
        <CardTitle>{{ t('host-game') }}</CardTitle>
      </CardHeader>

      <CardContent>
        <Label>
          <span>{{ t('world-name') }}</span>
          <Input
            v-model.trim="worldOptions.name"
            type="text"
            :disabled="locked"
            :minlength="1"
            :maxlength="30"
          />
        </Label>

        <Label>
          <span>{{ t('world-size') }}</span>
          <NumberField
            v-model="worldOptions.size"
            :disabled="locked"
            :min="100"
            :max="200"
            :step="10"
            class="w-full"
          >
            <NumberFieldContent>
              <NumberFieldDecrement />
              <NumberFieldInput class="dark:bg-input/40" />
              <NumberFieldIncrement />
            </NumberFieldContent>
          </NumberField>
        </Label>

        <Label>
          <span>{{ t('world-password') }}</span>
          <Input
            v-model="worldPassword"
            type="password"
            :disabled="locked"
            :minlength="3"
            :maxlength="50"
          />
        </Label>

        <Label>
          <span>{{ t('world-description') }}</span>
          <Textarea
            v-model.trim="description"
            type="text"
            spellcheck="false"
            autocapitalize="off"
            autocomplete="off"
            class="max-h-24 resize-none"
          />
        </Label>

        <div class="flex items-center justify-center py-1">
          <Label>
            <Checkbox v-model="worldOptions.allowCheats" :disabled="locked" />
            <span>{{ t('allow-cheats') }}</span>
          </Label>
        </div>
      </CardContent>

      <CardFooter class="grid grid-cols-2">
        <Button :disabled="locked || !canHost" @click="host">
          <span>{{ t('host') }}</span>
        </Button>

        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
