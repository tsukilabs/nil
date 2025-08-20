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
import { usePublicFields } from '@/composables/continent/usePublicFields';
import { useDefaultCoords } from '@/composables/continent/useDefaultCoords';
import { useContinentGrid } from '@/composables/continent/useContinentGrid';
import { useMouseMovement } from '@/composables/continent/useMouseMovement';
import { useKeyboardMovement } from '@/composables/continent/useKeyboardMovement';
import { nextTick, onBeforeMount, onMounted, onUnmounted, useTemplateRef, watchEffect } from 'vue';

const continent = new Continent();

const { fields, setCoords, loadCoord } = usePublicFields();

const route = useRoute();
const defaultCoords = useDefaultCoords();

const containerEl = useTemplateRef('container');
const containerSize = useElementSize(containerEl);

const { gridCols, gridRows, cols, rows } = useContinentGrid(fields);

const listener = new ListenerSet();
listener.event.onPublicCityUpdated(({ coord }) => loadCoord(coord));

watchEffect(() => {
  const width = containerSize.width.value;
  const height = containerSize.height.value;
  continent.set_container_size(width, height);

  void nextTick(() => {
    gridCols.value = continent.grid_cols();
    gridRows.value = continent.grid_rows();
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
  requestIdleCallback(updateCoordsWithin);
});

onUnmounted(() => {
  continent.free();
});

useKeyboardMovement(continent, render);
useMouseMovement(continent, render);

function render() {
  if (route.name === ('continent' satisfies GameScene)) {
    requestAnimationFrame(updateCoordsWithin);
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
        <div
          ref="container"
          class="bg-card absolute inset-0 bottom-[50px] left-[50px] z-10 overflow-hidden"
        >
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

        <div id="container-fill" class="bg-accent"></div>
      </CardContent>
    </Card>
  </div>
</template>

<style scoped>
#continent-grid {
  display: grid;
  grid-template-rows: v-bind("`repeat(${gridRows}, 50px)`");
  grid-template-columns: v-bind("`repeat(${gridCols}, 50px)`");
}

#container-fill {
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
  left: 50px;
  grid-template-columns: v-bind("`repeat(${gridCols}, 50px)`");
  height: 50px;
}

#rule-vertical {
  top: 0;
  bottom: 50px;
  grid-template-rows: v-bind("`repeat(${gridRows}, 50px)`");
  width: 50px;
}
</style>
