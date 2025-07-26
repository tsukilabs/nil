<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { Checkbox, Label } from '@tb-dev/vue-components';
import { useAcademySettings } from '@/settings/infrastructure/academy';

const settings = useAcademySettings();
const { hideUnmet } = storeToRefs(settings);
</script>

<template>
  <div class="size-full px-4">
    <div class="flex flex-col">
      <Label>
        <Checkbox v-model="hideUnmet" />
        <span>unmet</span>
      </Label>
    </div>
  </div>
</template>
