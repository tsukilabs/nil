<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Label } from '@ui/label';
import { useI18n } from 'vue-i18n';
import Section from './Section.vue';
import { Button } from '@ui/button';
import { DESKTOP } from '@/lib/global';
import { useRouter } from 'vue-router';
import { Checkbox } from '@ui/checkbox';
import { useBreakpoints } from '@tb-dev/vue';
import { useSettings } from '@/stores/settings';
import enUS from '@/locale/en-US/scenes/settings.json';
import ptBR from '@/locale/pt-BR/scenes/settings.json';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@ui/card';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@ui/select';

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
    <Card :key="settings.general.locale" class="max-md:size-full md:max-h-[95%] overflow-hidden">
      <CardHeader>
        <CardTitle>{{ t('settings') }}</CardTitle>
      </CardHeader>

      <CardContent class="w-full flex flex-col gap-4 overflow-auto max-md:px-2">
        <Section :title="t('general')" title-id="general">
          <div class="w-full flex flex-col gap-2">
            <div v-if="DESKTOP" class="flex flex-col items-start justify-center gap-1 py-1">
              <Label>
                <Checkbox v-model="settings.general.hideOnClose" />
                <span>{{ t('hide-on-close') }}</span>
              </Label>
              <Label>
                <Checkbox v-model="settings.general.autoUpdate" />
                <span>{{ t('auto-update') }}</span>
              </Label>
            </div>

            <Label>
              <span>{{ t('language') }}</span>
              <Select v-model="settings.general.locale">
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
          <div class="w-full flex flex-col gap-2">
            <Label>
              <span>{{ t('mode') }}</span>
              <Select v-model="settings.appearance.colorMode">
                <SelectTrigger class="w-full">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="light">{{ t('light') }}</SelectItem>
                  <SelectItem value="dark">{{ t('dark') }}</SelectItem>
                </SelectContent>
              </Select>
            </Label>
          </div>
        </Section>
      </CardContent>

      <CardFooter class="w-full flex justify-center items-center">
        <Button
          variant="default"
          :size="sm ? 'default' : 'sm'"
          class="px-8"
          @click="() => router.back()"
        >
          <span>{{ t('back') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
