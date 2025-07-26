<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { Checkbox, Label } from '@tb-dev/vue-components';
import { useStableSettings } from '@/settings/infrastructure/stable';

const settings = useStableSettings();
const { hideUnmet } = storeToRefs(settings);
</script>

<template>
  <div class="size-full px-4">
    <div class="flex flex-col gap-4">
      <Label>
        <Checkbox v-model="hideUnmet" />
        <span>unmet</span>
      </Label>
    </div>
  </div>
</template>
