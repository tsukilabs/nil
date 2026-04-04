<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { Input } from '@ui/input';
import { Label } from '@ui/label';
import { useI18n } from 'vue-i18n';

defineProps<{
  disabled: boolean;
}>();

const server = defineModel<Option<string>>({ required: true });

const { t } = useI18n();
</script>

<template>
  <Label>
    <span>{{ t('server') }}</span>
    <Input
      v-model="server"
      type="text"
      :disabled
      :minlength="1"
      :maxlength="50"
    />
  </Label>
</template>
