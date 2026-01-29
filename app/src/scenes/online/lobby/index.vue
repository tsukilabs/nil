<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { watchToken } from '@/composables/online/watchToken';

watchToken('sign-in');
</script>

<template>
  <div class="card-layout">lobby</div>
</template>
