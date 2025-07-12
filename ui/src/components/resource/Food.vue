<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Base from './Base.vue';
import { useI18n } from 'vue-i18n';
import type { Option } from '@tb-dev/utils';

defineProps<{
  amount?: Option<number>;
  limit?: Option<number>;
}>();

const { t } = useI18n();
</script>

<template>
  <Base :amount :limit :name="t('food')" color="oklch(0.7 0.1751 133.41)" />
</template>
