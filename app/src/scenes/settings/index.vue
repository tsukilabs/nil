<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import Section from './Section.vue';
import { DESKTOP } from '@/lib/global';
import { useRouter } from 'vue-router';
import { useSettings } from '@/settings';
import enUS from '@/locale/en-US/scenes/settings.json';
import ptBR from '@/locale/pt-BR/scenes/settings.json';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  Checkbox,
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
const settings = useSettings();

const { sm, md } = useBreakpoints();
</script>

<template>
  <div :class="md ? 'card-layout' : 'game-layout'">
    <Card :key="settings.locale" class="max-md:size-full">
      <CardHeader>
        <CardTitle>
          <div class="flex items-center justify-between">
            <span>{{ t('settings') }}</span>
            <Button
              variant="default"
              :size="sm ? 'default' : 'sm'"
              class="px-8"
              @click="() => router.back()"
            >
              <span>{{ t('back') }}</span>
            </Button>
          </div>
        </CardTitle>
      </CardHeader>

      <CardContent class="w-full flex flex-col gap-4 overflow-auto md:pb-8">
        <Section :title="t('general')" title-id="general">
          <div class="w-full max-md:max-w-72 flex flex-col gap-4">
            <div v-if="DESKTOP" class="flex flex-col items-start justify-center gap-1 py-1">
              <Label>
                <Checkbox v-model="settings.hideOnClose" />
                <span>{{ t('hide-on-close') }}</span>
              </Label>
              <Label>
                <Checkbox v-model="settings.autoUpdate" />
                <span>{{ t('auto-update') }}</span>
              </Label>
            </div>

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
          </div>
        </Section>

        <Section :title="t('appearance')" title-id="appearance">
          <div class="w-full max-md:max-w-72 flex flex-col gap-4">
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
          </div>
        </Section>
      </CardContent>
    </Card>
  </div>
</template>
