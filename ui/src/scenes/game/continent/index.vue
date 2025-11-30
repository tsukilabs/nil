<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Field from './Field.vue';
import { until } from '@vueuse/core';
import { useRoute } from 'vue-router';
import Navigation from './Navigation.vue';
import { useElementSize } from '@tb-dev/vue';
import { ListenerSet } from '@/lib/listener-set';
import { Continent } from '@tsukilabs/nil-continent';
import { CoordImpl } from '@/core/model/continent/coord';
import { Card, CardContent } from '@tb-dev/vue-components';
import { useBreakpoints } from '@/composables/util/useBreakpoints';
import { PublicFieldImpl } from '@/core/model/continent/public-field';
import { memory } from '@tsukilabs/nil-continent/nil_continent_bg.wasm';
import { useQueryCoords } from '@/composables/continent/useQueryCoords';
import { type Direction, onKeyboardMovement } from '@/composables/continent/onKeyboardMovement';
import {
  computed,
  nextTick,
  onBeforeMount,
  onMounted,
  onUnmounted,
  ref,
  shallowRef,
  triggerRef,
  useTemplateRef,
  watchEffect,
} from 'vue';

const continent = new Continent();

const route = useRoute();
const queryCoords = useQueryCoords();
const { continentSize } = NIL.world.refs();

const fields = shallowRef<PublicFieldImpl[]>([]);
const cache = new Map<string, PublicFieldImpl>();
const bulkInit = PublicFieldImpl.createBulkInitializer();
const triggerFields = () => triggerRef(fields);

const containerEl = useTemplateRef('container');
const containerSize = useElementSize(containerEl);

const maxCols = ref(0);
const maxRows = ref(0);

const { sm } = useBreakpoints();
const cellSize = computed(() => sm.value ? 50 : 30);

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
  const x = queryCoords.value?.x;
  const y = queryCoords.value?.y;
  if (typeof x === 'number' && typeof y === 'number') {
    continent.set_center(x, y);
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
  fields.value = [];
  cache.clear();
  continent.free();
});

if (__DESKTOP__) {
  onKeyboardMovement(move);
}

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

function setCoords(coords: CoordImpl[]) {
  const values: PublicFieldImpl[] = [];
  for (const coord of coords) {
    let field = cache.get(coord.id);
    if (!field) {
      field = PublicFieldImpl.create(coord);
      cache.set(coord.id, field);
    }

    values.push(field);
  }

  initFields(values).err();
  fields.value = values;
}

async function initFields(values: PublicFieldImpl[]) {
  if (await bulkInit(values) > 0) {
    triggerFields();
  }
}

async function loadCoord(coord: Coord) {
  const field = fields.value.find((it) => it.coord.is(coord));
  if (field && !field.isLoading()) {
    // When loading begins, `PublicFieldImpl` modifies its flags to indicate that an update is taking place.
    // Since Vue relies on these flags to detect changes (due to the use of `key` in the grid loop),
    // we need to trigger the `fields` ref both before and after the update is completed.
    //
    // If the ref were only triggered at the end, Vue would not update the grid because the value
    // of the flag would most likely have already reverted to its initial state by that time.
    await field.load({
      onBeforeLoad: triggerFields,
      onLoad: triggerFields,
    });
  }
}

function move(direction: Direction, delta: number) {
  const center = continent.center();
  const initialX = center.x();
  const initialY = center.y();

  let x = initialX;
  let y = initialY;

  switch (direction) {
    case 'up': {
      y = Math.min(y + delta, continentSize.value - 1);
      break;
    }
    case 'down': {
      y = Math.max(y - delta, 0);
      break;
    }
    case 'left': {
      x = Math.max(x - delta, 0);
      break;
    }
    case 'right': {
      x = Math.min(x + delta, continentSize.value - 1);
      break;
    }
  }

  if (x !== initialX || y !== initialY) {
    continent.set_center(x, y);
    render();
  }
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full overflow-hidden p-0">
      <CardContent class="relative size-full overflow-hidden p-0 select-none">
        <div id="continent-container" ref="container" class="bg-card">
          <Navigation
            :cell-size
            :interval="50"
            @up="() => move('up', 1)"
            @right="() => move('right', 1)"
            @down="() => move('down', 1)"
            @left="() => move('left', 1)"
          />

          <div id="continent-grid">
            <Field
              v-for="field of fields"
              :key="`${field.id}-${field.flags}`"
              :field
            />
          </div>
        </div>

        <div id="rule-horizontal" class="rule bg-accent font-nil text-xs sm:text-lg">
          <div v-for="col of cols" :key="col[0]">
            <span v-if="typeof col[1] === 'number'">{{ col[1] }}</span>
          </div>
        </div>

        <div id="rule-vertical" class="rule bg-accent font-nil text-xs sm:text-lg">
          <div v-for="row of rows" :key="row[0]">
            <span v-if="typeof row[1] === 'number'">{{ row[1] }}</span>
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
  user-select: none;
  touch-action: none;
}

#continent-grid {
  display: grid;
  grid-template-rows: v-bind("`repeat(${maxRows}, ${cellSize}px)`");
  grid-template-columns: v-bind("`repeat(${maxCols}, ${cellSize}px)`");
  user-select: none;
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
  user-select: none;
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
