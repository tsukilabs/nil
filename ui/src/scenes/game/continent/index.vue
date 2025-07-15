<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Field from './Field.vue';
import { useRoute } from 'vue-router';
import type { Option } from '@tb-dev/utils';
import { useElementSize } from '@tb-dev/vue';
import { Card } from '@tb-dev/vue-components';
import { getContinentSize } from '@/commands';
import { CoordImpl } from '@/core/model/coord';
import { onKeyDown, until } from '@vueuse/core';
import { ListenerSet } from '@/lib/listener-set';
import { memory } from 'nil-continent/nil_continent_bg.wasm';
import { Continent, Coord as WasmCoord } from 'nil-continent';
import { PublicFieldImpl } from '@/core/model/continent/public-field';
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

const route = useRoute();
const { coord: currentCoord } = NIL.village.refs();

const continent = new Continent();
const size = await getContinentSize();

const fields = shallowRef<PublicFieldImpl[]>([]);
const cache = new Map<string, PublicFieldImpl>();
const bulkInit = PublicFieldImpl.createBulkInitializer(size);
const triggerFields = () => triggerRef(fields);

const containerEl = useTemplateRef('container');
const containerSize = useElementSize(containerEl);

const gridCols = ref(0);
const gridRows = ref(0);

const cols = computed(() => {
  const values: [number, Option<number>][] = [];
  for (let col = 0; col < gridCols.value; col++) {
    const field = fields.value.at(col);
    if (field && !field.isXOutside(size)) {
      values.push([col, field.x]);
    } else {
      values.push([col, null]);
    }
  }

  return values;
});

const rows = computed(() => {
  const values: [number, Option<number>][] = [];
  for (let row = 0; row < gridRows.value; row++) {
    const field = fields.value.at(row * gridCols.value);
    if (field && !field.isYOutside(size)) {
      values.push([row, field.y]);
    } else {
      values.push([row, null]);
    }
  }

  return values;
});

const listener = new ListenerSet();
listener.event.onPublicVillageUpdated(async ({ coord }) => {
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
});

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
  if (currentCoord.value) {
    const x = currentCoord.value.x;
    const y = currentCoord.value.y;
    continent.set_center(new WasmCoord(x, y));
  }
});

onMounted(async () => {
  await until(containerEl).toBeTruthy();
  requestIdleCallback(updateCoordsWithin);
});

onUnmounted(() => {
  cache.clear();
  continent.free();
});

onKeyDown('ArrowUp', move('up'), { dedupe: false });
onKeyDown('ArrowDown', move('down'), { dedupe: false });
onKeyDown('ArrowLeft', move('left'), { dedupe: false });
onKeyDown('ArrowRight', move('right'), { dedupe: false });

function render() {
  if (route.name === ('continent' satisfies GameScene)) {
    requestAnimationFrame(updateCoordsWithin);
  }
}

function updateCoordsWithin() {
  const ptr = continent.update_coords_within();
  const len = continent.coords_byte_length();
  const view = new DataView(memory.buffer, ptr, len);

  const result: PublicFieldImpl[] = [];
  for (let i = 0; i < len; i += 4) {
    const x = view.getInt16(i, true);
    const y = view.getInt16(i + 2, true);
    const coord = CoordImpl.create({ x, y });
    result.push(getCachedField(coord));
  }

  bulkInit(result)
    .then((counter) => counter && triggerFields())
    .err();

  fields.value = result;
}

function getCachedField(coord: CoordImpl) {
  let field = cache.get(coord.id);
  if (!field) {
    field = PublicFieldImpl.create(coord);
    cache.set(coord.id, field);
  }

  return field;
}

function move(dir: 'up' | 'down' | 'left' | 'right') {
  return function (e: KeyboardEvent) {
    const center = continent.center();
    let x = center.x();
    let y = center.y();

    let delta = 1;
    if (e.ctrlKey) delta = 5;
    if (e.shiftKey) delta = 10;
    if (e.ctrlKey && e.shiftKey) delta = 25;

    if (dir === 'up' && y + delta <= size) {
      y += delta;
    } else if (dir === 'down' && y - delta >= 0) {
      y -= delta;
    } else if (dir === 'left' && x - delta >= 0) {
      x -= delta;
    } else if (dir === 'right' && x + delta <= size) {
      x += delta;
    }

    continent.set_center(new WasmCoord(x, y));

    render();
  };
}
</script>

<template>
  <div class="game-layout">
    <Card
      class="size-full overflow-hidden p-0"
      content-class="relative size-full overflow-hidden p-0 select-none"
    >
      <div
        ref="container"
        class="bg-card absolute inset-0 bottom-[50px] left-[50px] z-10 overflow-hidden"
      >
        <div id="continent-grid">
          <Field
            v-for="field of fields"
            :key="`${field.id}-${field.flags}`"
            :field
            :continent-size="size"
          />
        </div>
      </div>

      <div id="rule-horizontal" class="rule bg-accent font-nil text-lg">
        <div v-for="[idx, col] of cols" :key="idx">
          <span v-if="typeof col === 'number'">{{ col }}</span>
        </div>
      </div>

      <div id="rule-vertical" class="rule bg-accent font-nil text-lg">
        <div v-for="[idx, row] of rows" :key="idx">
          <span v-if="typeof row === 'number'">{{ row }}</span>
        </div>
      </div>

      <div id="container-fill" class="bg-accent"></div>
    </Card>
  </div>
</template>

<style scoped>
#continent-grid {
  display: grid;
  grid-template-rows: v-bind('`repeat(${gridRows}, 50px)`');
  grid-template-columns: v-bind('`repeat(${gridCols}, 50px)`');
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
  grid-template-columns: v-bind('`repeat(${gridCols}, 50px)`');
  height: 50px;
}

#rule-vertical {
  top: 0;
  bottom: 50px;
  grid-template-rows: v-bind('`repeat(${gridRows}, 50px)`');
  width: 50px;
}
</style>
