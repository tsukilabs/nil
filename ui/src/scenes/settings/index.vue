<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { useGlobalSettings } from '@/settings/global';
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
const settings = useGlobalSettings();
</script>

<template>
  <div class="card-layout">
    <Card :key="settings.locale">
      <CardHeader>
        <CardTitle>{{ t('settings') }}</CardTitle>
      </CardHeader>

      <CardContent>
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

      <CardFooter class="flex">
        <Button variant="default" class="px-8" @click="() => router.back()">
          <span>OK</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
