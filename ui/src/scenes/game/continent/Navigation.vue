<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { computed } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { Button } from '@tb-dev/vue-components';
import type { Direction } from '@/composables/continent/onKeyboardMovement';
import { ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon, ChevronUpIcon } from 'lucide-vue-next';

const props = defineProps<{
  cellSize: number;
  interval: number;
  onUp: () => void;
  onRight: () => void;
  onDown: () => void;
  onLeft: () => void;
}>();

const offset = computed(() => toPixel(Math.round(props.cellSize / 2)));

type Timeout = ReturnType<typeof setInterval>;
let upInterval: Option<Timeout> = null;
let rightInterval: Option<Timeout> = null;
let downInterval: Option<Timeout> = null;
let leftInterval: Option<Timeout> = null;

const onUpStart = onStart('up');
const onRightStart = onStart('right');
const onDownStart = onStart('down');
const onLeftStart = onStart('left');

const onUpEnd = onEnd('up');
const onRightEnd = onEnd('right');
const onDownEnd = onEnd('down');
const onLeftEnd = onEnd('left');

function onStart(direction: Direction) {
  return function(e: MouseEvent | TouchEvent) {
    e.preventDefault();
    stop(direction);

    switch (direction) {
      case 'up': {
        props.onUp();
        upInterval = start(() => props.onUp());
        break;
      }
      case 'right': {
        props.onRight();
        rightInterval = start(() => props.onRight());
        break;
      }
      case 'down': {
        props.onDown();
        downInterval = start(() => props.onDown());
        break;
      }
      case 'left': {
        props.onLeft();
        leftInterval = start(() => props.onLeft());
        break;
      }
    }
  };
}

function onEnd(direction: Direction) {
  return function(e: MouseEvent | TouchEvent) {
    e.preventDefault();
    stop(direction);
  };
}

function start(fn: () => void) {
  return setInterval(fn, props.interval);
}

function stop(direction: Direction) {
  switch (direction) {
    case 'up': {
      if (upInterval) {
        clearInterval(upInterval);
        upInterval = null;
      }
      break;
    }
    case 'right': {
      if (rightInterval) {
        clearInterval(rightInterval);
        rightInterval = null;
      }
      break;
    }
    case 'down': {
      if (downInterval) {
        clearInterval(downInterval);
        downInterval = null;
      }
      break;
    }
    case 'left': {
      if (leftInterval) {
        clearInterval(leftInterval);
        leftInterval = null;
      }
      break;
    }
  }
}
</script>

<template>
  <Button
    id="continent-nav-up"
    variant="ghost"
    size="icon"
    class="continent-nav"
    @mousedown="onUpStart"
    @mouseleave="onUpEnd"
    @mouseup="onUpEnd"
    @touchstart="onUpStart"
    @touchend="onUpEnd"
  >
    <ChevronUpIcon />
  </Button>
  <Button
    id="continent-nav-right"
    variant="ghost"
    size="icon"
    class="continent-nav"
    @mousedown="onRightStart"
    @mouseleave="onRightEnd"
    @mouseup="onRightEnd"
    @touchstart="onRightStart"
    @touchend="onRightEnd"
  >
    <ChevronRightIcon />
  </Button>
  <Button
    id="continent-nav-down"
    variant="ghost"
    size="icon"
    class="continent-nav"
    @mousedown="onDownStart"
    @mouseleave="onDownEnd"
    @mouseup="onDownEnd"
    @touchstart="onDownStart"
    @touchend="onDownEnd"
  >
    <ChevronDownIcon />
  </Button>
  <Button
    id="continent-nav-left"
    variant="ghost"
    size="icon"
    class="continent-nav"
    @mousedown="onLeftStart"
    @mouseleave="onLeftEnd"
    @mouseup="onLeftEnd"
    @touchstart="onLeftStart"
    @touchend="onLeftEnd"
  >
    <ChevronLeftIcon />
  </Button>
</template>

<style scoped>
.continent-nav {
  position: absolute;
  z-index: 20;
  opacity: 60%;
}

#continent-nav-up {
  top: 0.25rem;
  left: v-bind("`calc(50% - ${offset})`");
}

#continent-nav-right {
  top: v-bind("`calc(50% - ${offset})`");
  right: 0.25rem;
}

#continent-nav-down {
  bottom: 0.25rem;
  left: v-bind("`calc(50% - ${offset})`");
}

#continent-nav-left {
  top: v-bind("`calc(50% - ${offset})`");
  left: 0.25rem;
}
</style>
