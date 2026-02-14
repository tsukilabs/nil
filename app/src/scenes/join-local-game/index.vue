<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { joinLocalGame } from '@/core/game';
import { isPlayerOptions } from '@/lib/schema';
import { SocketAddrV4 } from '@/lib/net/addr-v4';
import { localRef, useMutex } from '@tb-dev/vue';
import InputPlayerName from '@/components/form/InputPlayerName.vue';
import type { WithPartialNullish, WritablePartial } from '@tb-dev/utils';
import InputServerAddress from '@/components/form/InputServerAddress.vue';
import { Button, Card, CardContent, CardFooter, CardHeader, CardTitle } from '@tb-dev/vue-components';

const { t } = useI18n();

const router = useRouter();

const playerOptions = localRef<WritablePartial<PlayerOptions>>(
  'join-local-game:player',
  {
    id: null,
  } satisfies WithPartialNullish<PlayerOptions, 'id'>,
);

const server = localRef<string>('join-local-game:server', '');
const serverAddr = computed<Option<ServerAddr>>(() => {
  const addr = SocketAddrV4.tryParse(server.value);
  if (addr) {
    return { kind: 'local', addr: addr.format() };
  }
  else {
    return null;
  }
});

const { locked, lock } = useMutex();
const canJoin = computed(() => {
  return isPlayerOptions(playerOptions.value) && serverAddr.value;
});

async function join() {
  await lock(async () => {
    if (isPlayerOptions(playerOptions.value) && serverAddr.value) {
      await joinLocalGame({
        serverAddr: serverAddr.value,
        playerId: playerOptions.value.id,
      });
    }
  });
}
</script>

<template>
  <div class="card-layout">
    <Card>
      <CardHeader>
        <CardTitle>{{ t('join-game') }}</CardTitle>
      </CardHeader>

      <CardContent>
        <InputPlayerName v-model="playerOptions.id" :disabled="locked" />
        <InputServerAddress v-model="server" :disabled="locked" />
      </CardContent>

      <CardFooter class="grid grid-cols-2">
        <Button :disabled="!canJoin || locked" @click="join">
          {{ t('join') }}
        </Button>
        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t('cancel') }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
