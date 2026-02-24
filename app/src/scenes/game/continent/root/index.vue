<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import Menu from './Menu.vue';
import Field from './Field.vue';
import { until } from '@vueuse/core';
import { useRoute } from 'vue-router';
import Navigation from './Navigation.vue';
import { CoordImpl } from '@/core/model/continent/coord';
import { Card, CardContent } from '@tb-dev/vue-components';
import { useBreakpoints, useElementSize } from '@tb-dev/vue';
import { useContinent } from '@/composables/continent/useContinent';
import { useQueryCoord } from '@/composables/continent/useQueryCoord';
import { computed, nextTick, onBeforeMount, onMounted, useTemplateRef, watch } from 'vue';
import { type Direction, onKeyboardMovement } from '@/composables/continent/onKeyboardMovement';

const route = useRoute();
const { initialCoord, updateQueryCoords } = useQueryCoord();

const { continentSize } = NIL.world.refs();
const { coord: currentCoord } = NIL.city.refs();

const containerEl = useTemplateRef('container');
const containerSize = useElementSize(containerEl);

const { sm } = useBreakpoints();
const cellSize = computed(() => sm.value ? 50 : 30);

const {
  center,
  fields,
  cols,
  rows,
  maxCols,
  maxRows,
  updateCoordsWithin,
} = useContinent({
  width: containerSize.width,
  height: containerSize.height,
  cellSize,
});

watch([containerSize.width, containerSize.height, cellSize], render);

onBeforeMount(() => {
  const x = initialCoord.value?.x;
  const y = initialCoord.value?.y;
  if (typeof x === 'number' && typeof y === 'number') {
    center.value = CoordImpl.create({ x, y });
  }
});

onMounted(async () => {
  await until(containerEl).toBeTruthy();
  await nextTick(() => render());
});

if (__DESKTOP__) {
  onKeyboardMovement(move);
}

function render() {
  if (route.name === ('continent' satisfies GameScene)) {
    if ('requestAnimationFrame' in window) {
      requestAnimationFrame(updateCoordsWithin);
    }
    else if ('requestIdleCallback' in window) {
      requestIdleCallback(updateCoordsWithin);
    }
    else {
      updateCoordsWithin();
    }
  }
}

function move(direction: Direction, delta: number) {
  const initialX = center.value.x;
  const initialY = center.value.y;

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
    center.value = CoordImpl.create({ x, y });
    updateQueryCoords(x, y);
    render();
  }
}
</script>

<template>
  <div class="game-layout">
    <Card class="size-full overflow-hidden p-0">
      <CardContent class="relative size-full overflow-hidden p-0 select-none">
        <Menu />

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
              :current-coord
            />
          </div>
        </div>

        <div id="rule-horizontal" class="rule bg-accent font-syne-mono text-xs sm:text-lg">
          <div v-for="col of cols" :key="col[0]">
            <span v-if="typeof col[1] === 'number'">{{ col[1] }}</span>
          </div>
        </div>

        <div id="rule-vertical" class="rule bg-accent font-syne-mono text-xs sm:text-lg">
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
