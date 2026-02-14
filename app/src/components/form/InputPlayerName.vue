<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Input, Label } from '@tb-dev/vue-components';

defineProps<{
  disabled: boolean;
}>();

const name = defineModel<Option<string>>({ required: true });

const { t } = useI18n();
</script>

<template>
  <Label>
    <span>{{ t('player-name') }}</span>
    <Input
      v-model="name"
      type="text"
      :disabled
      :minlength="1"
      :maxlength="20"
    />
  </Label>
</template>
