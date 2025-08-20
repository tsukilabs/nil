<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Field from './Field.vue';
import { until } from '@vueuse/core';
import { useRoute } from 'vue-router';
import { useElementSize } from '@tb-dev/vue';
import { ListenerSet } from '@/lib/listener-set';
import { CoordImpl } from '@/core/model/continent/coord';
import { Card, CardContent } from '@tb-dev/vue-components';
import { memory } from 'nil-continent/nil_continent_bg.wasm';
import { Continent, Coord as WasmCoord } from 'nil-continent';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { usePublicFields } from '@/composables/continent/usePublicFields';
import { onMouseMovement } from '@/composables/continent/onMouseMovement';
import { useDefaultCoords } from '@/composables/continent/useDefaultCoords';
import { onKeyboardMovement } from '@/composables/continent/onKeyboardMovement';
import { computed, nextTick, onBeforeMount, onMounted, onUnmounted, ref, useTemplateRef, watchEffect } from 'vue';

const continent = new Continent();

const { fields, setCoords, loadCoord } = usePublicFields();

const route = useRoute();
const defaultCoords = useDefaultCoords();

const containerEl = useTemplateRef('container');
const containerSize = useElementSize(containerEl);

const maxCols = ref(0);
const maxRows = ref(0);

const { sm } = useBreakpoints();
const cellSize = computed(() => sm.value ? 50 : 25);

const cols = computed(() => {
  const values: [number, Option<number>][] = [];
  for (let col = 0; col < maxCols.value; col++) {
    const field = fields.value.at(col);
    if (field && !field.isXOutside()) {
      values.push([col, field.x]);
    }
    else {
      values.push([col, null]);
    }
  }

  return values;
});

const rows = computed(() => {
  const values: [number, Option<number>][] = [];
  for (let row = 0; row < maxRows.value; row++) {
    const field = fields.value.at(row * maxCols.value);
    if (field && !field.isYOutside()) {
      values.push([row, field.y]);
    }
    else {
      values.push([row, null]);
    }
  }

  return values;
});

const listener = new ListenerSet();
listener.event.onPublicCityUpdated(({ coord }) => loadCoord(coord));

watchEffect(() => {
  const width = containerSize.width.value;
  const height = containerSize.height.value;
  continent.set_container_size(width, height);
  continent.set_cell_size(cellSize.value);

  void nextTick(() => {
    maxCols.value = continent.max_cols();
    maxRows.value = continent.max_rows();
    render();
  });
});

onBeforeMount(() => {
  const x = defaultCoords.value?.x;
  const y = defaultCoords.value?.y;
  if (typeof x === 'number' && typeof y === 'number') {
    continent.set_center(new WasmCoord(x, y));
  }
});

onMounted(async () => {
  await until(containerEl).toBeTruthy();
  if ('requestIdleCallback' in window) {
    requestIdleCallback(updateCoordsWithin);
  }
  else {
    updateCoordsWithin();
  }
});

onUnmounted(() => {
  continent.free();
});

onKeyboardMovement(continent, render);
onMouseMovement(containerEl, continent, render);

function render() {
  if (route.name === ('continent' satisfies GameScene)) {
    if ('requestAnimationFrame' in window) {
      requestAnimationFrame(updateCoordsWithin);
    }
    else {
      updateCoordsWithin();
    }
  }
}

function updateCoordsWithin() {
  const ptr = continent.update_coords_within();
  const len = continent.coords_byte_length();
  const view = new DataView(memory.buffer, ptr, len);

  const coords: CoordImpl[] = [];
  for (let i = 0; i < len; i += 4) {
    const x = view.getInt16(i, true);
    const y = view.getInt16(i + 2, true);
    coords.push(CoordImpl.create({ x, y }));
  }

  setCoords(coords);
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full overflow-hidden p-0">
      <CardContent class="relative size-full overflow-hidden p-0 select-none">
        <div id="continent-container" ref="container" class="bg-card">
          <div id="continent-grid">
            <Field v-for="field of fields" :key="`${field.id}-${field.flags}`" :field />
          </div>
        </div>

        <div id="rule-horizontal" class="rule bg-accent font-nil text-lg">
          <div v-for="([idx, col]) of cols" :key="idx">
            <span v-if="typeof col === 'number'">{{ col }}</span>
          </div>
        </div>

        <div id="rule-vertical" class="rule bg-accent font-nil text-lg">
          <div v-for="([idx, row]) of rows" :key="idx">
            <span v-if="typeof row === 'number'">{{ row }}</span>
          </div>
        </div>

        <div id="continent-background" class="bg-accent"></div>
      </CardContent>
    </Card>
  </div>
</template>

<style scoped>
#continent-container {
  position: absolute;
  inset: 0;
  bottom: v-bind("`${cellSize}px`");
  left: v-bind("`${cellSize}px`");
  z-index: 10;
  overflow: hidden;
}

#continent-grid {
  display: grid;
  grid-template-rows: v-bind("`repeat(${maxRows}, ${cellSize}px)`");
  grid-template-columns: v-bind("`repeat(${maxCols}, ${cellSize}px)`");
}

#continent-background {
  position: absolute;
  z-index: 0;
  inset: 0;
}

.rule {
  display: grid;
  position: absolute;
  justify-content: center;
  align-items: center;
  z-index: 5;
  overflow: hidden;
}

.rule > div {
  text-align: center;
}

#rule-horizontal {
  bottom: 0;
  left: v-bind("`${cellSize}px`");
  grid-template-columns: v-bind("`repeat(${maxCols}, ${cellSize}px)`");
  height: v-bind("`${cellSize}px`");
}

#rule-vertical {
  top: 0;
  bottom: v-bind("`${cellSize}px`");
  grid-template-rows: v-bind("`repeat(${maxRows}, ${cellSize}px)`");
  width: v-bind("`${cellSize}px`");
}
</style>
