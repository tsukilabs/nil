<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { asyncRef } from '@tb-dev/vue';
import * as commands from '@/commands';
import { Button, Card, CardContent, CardFooter } from '@tb-dev/vue-components';

const router = useRouter();

const { state: version } = asyncRef(null, commands.version);
</script>

<template>
  <div class="card-layout">
    <Card>
      <CardContent>
        <div class="flex flex-col items-center justify-center p-4">
          <h1 class="font-bold text-2xl">Call of Nil</h1>
          <h2 v-if="version" class="text-muted-foreground">
            {{ version }}
          </h2>
        </div>
      </CardContent>

      <CardFooter class="flex">
        <Button variant="default" class="px-8" @click="() => router.back()">
          <span>OK</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
