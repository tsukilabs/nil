<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { useSettingsStore } from '@/stores/settings';
import enUS from '@/locale/en-US/scenes/settings.json';
import ptBR from '@/locale/pt-BR/scenes/settings.json';
import {
  Button,
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@tb-dev/vue-components';

const { t } = useI18n({
  messages: {
    'en-US': enUS,
    'pt-BR': ptBR,
  },
});

const router = useRouter();
const settings = useSettingsStore();
</script>

<template>
  <div class="flex size-full flex-col items-center justify-center gap-2">
    <Card :key="settings.locale" class="p-2 sm:min-w-80">
      <CardHeader class="pt-4">
        <CardTitle>
          <span class="text-xl">{{ t('settings') }}</span>
        </CardTitle>
      </CardHeader>

      <CardContent class="flex flex-col gap-4 px-4">
        <Label>
          <span>{{ t('language') }}</span>
          <Select v-model="settings.locale">
            <SelectTrigger class="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="en-US">English</SelectItem>
              <SelectItem value="pt-BR">Português</SelectItem>
            </SelectContent>
          </Select>
        </Label>

        <Label>
          <span>{{ t('mode') }}</span>
          <Select v-model="settings.colorMode">
            <SelectTrigger class="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="light">{{ t('light') }}</SelectItem>
              <SelectItem value="dark">{{ t('dark') }}</SelectItem>
            </SelectContent>
          </Select>
        </Label>

        <Label>
          <span>{{ t('theme') }}</span>
          <Select v-model="settings.theme">
            <SelectTrigger class="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="blue">{{ t('blue') }}</SelectItem>
              <SelectItem value="gray">{{ t('gray') }}</SelectItem>
              <SelectItem value="green">{{ t('green') }}</SelectItem>
              <SelectItem value="neutral">{{ t('neutral') }}</SelectItem>
              <SelectItem value="red">{{ t('red') }}</SelectItem>
              <SelectItem value="rose">{{ t('rose') }}</SelectItem>
              <SelectItem value="slate">{{ t('slate') }}</SelectItem>
              <SelectItem value="stone">{{ t('stone') }}</SelectItem>
              <SelectItem value="orange">{{ t('orange') }}</SelectItem>
              <SelectItem value="violet">{{ t('violet') }}</SelectItem>
              <SelectItem value="yellow">{{ t('yellow') }}</SelectItem>
              <SelectItem value="zinc">{{ t('zinc') }}</SelectItem>
            </SelectContent>
          </Select>
        </Label>
      </CardContent>

      <CardFooter class="flex items-center justify-center gap-2 px-12 pb-6">
        <Button to="home" variant="default" class="px-8" @click="() => router.back()">
          <span>OK</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
