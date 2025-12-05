<script setup lang="ts">
import type { Latest } from '../data/latest.data';

const props = defineProps<{ latest: Latest; }>();

function url(platform: keyof typeof props.latest.platforms) {
  return props.latest.platforms[platform].url;
}
</script>

<template>
  <table>
    <thead>
      <tr>
        <th>OS</th>
        <th></th>
      </tr>
    </thead>

    <tbody>
      <tr>
        <td>Windows</td>
        <td>
          <a :href="url('windows-x86_64-nsis')">Download</a>
        </td>
      </tr>
      <tr>
        <td>Linux (.deb)</td>
        <td>
          <a :href="url('linux-x86_64-deb')">Download</a>
        </td>
      </tr>
      <tr>
        <td>Linux (.rpm)</td>
        <td>
          <a :href="url('linux-x86_64-rpm')">Download</a>
        </td>
      </tr>
    </tbody>
  </table>
</template>
