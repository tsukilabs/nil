<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Base from './Base.vue';
import { useI18n } from 'vue-i18n';
import type { Option } from '@tb-dev/utils';

defineProps<{ amount?: Option<number> }>();

const { t } = useI18n();
</script>

<template>
  <Base :amount :name="t('workforce')" color="oklch(0.38 0.0493 83.65)" />
</template>
