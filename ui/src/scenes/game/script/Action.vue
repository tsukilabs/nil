<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import * as commands from '@/commands';
import * as dialog from '@/lib/dialog';
import { handleError } from '@/lib/error';
import { Button } from '@tb-dev/vue-components';
import type { MaybePromise, Option } from '@tb-dev/utils';

const props = defineProps<{
  current: Option<Script>;
  onBeforeSave: (id: ScriptId) => MaybePromise<unknown>;
  onImport: () => MaybePromise<unknown>;
  onRemove: (id: ScriptId) => MaybePromise<unknown>;
  onSave: (id: ScriptId) => MaybePromise<unknown>;
  waitToLoad: () => Promise<void>;
}>();

const loading = defineModel<boolean>('loading', { required: true });

const { t } = useI18n();

async function saveScript() {
  await props.waitToLoad();
  const script = props.current;
  if (script) {
    try {
      loading.value = true;
      await props.onBeforeSave(script.id);
      if (script.id > 0) {
        await commands.updateScript(script);
        await props.onSave(script.id);
      }
    } catch (err) {
      handleError(err);
    } finally {
      loading.value = false;
    }
  }
}

async function importScripts() {
  await props.waitToLoad();
  try {
    loading.value = true;
    const files = await dialog.open({
      directory: false,
      multiple: true,
      filters: [{ name: 'Lua', extensions: ['lua'] }],
    });

    if (Array.isArray(files) && files.length > 0) {
      const ids = await commands.importScripts(files);
      if (ids.length > 0) await props.onImport();
    }
  } catch (err) {
    handleError(err);
  } finally {
    loading.value = false;
  }
}

async function exportScripts() {
  await props.waitToLoad();
  const script = props.current;
  if (script) {
    try {
      loading.value = true;
      const dir = await dialog.open({
        directory: true,
        multiple: false,
      });

      if (dir) {
        await commands.exportScript(dir, script);
      }
    } catch (err) {
      handleError(err);
    } finally {
      loading.value = false;
    }
  }
}

async function removeScript() {
  await props.waitToLoad();
  const script = props.current;
  if (script) {
    try {
      loading.value = true;
      if (script.id > 0) {
        await commands.removeScript(script.id);
        await props.onRemove(script.id);
      }
    } catch (err) {
      handleError(err);
    } finally {
      loading.value = false;
    }
  }
}
</script>

<template>
  <div class="grid-cols-4 gap-2">
    <Button size="sm" :disabled="!current || loading" @click="saveScript">
      {{ t('save') }}
    </Button>
    <Button size="sm" :disabled="loading" @click="importScripts">
      {{ t('import') }}
    </Button>
    <Button size="sm" :disabled="!current || loading" @click="exportScripts">
      {{ t('export') }}
    </Button>
    <Button size="sm" variant="destructive" :disabled="!current || loading" @click="removeScript">
      {{ t('remove') }}
    </Button>
  </div>
</template>
