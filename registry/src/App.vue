<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script setup lang="ts">
import { asyncRef } from '@tb-dev/vue';

interface Entry {
  readonly id: string;
  readonly readme: string;
  readonly script: string;
  readonly frontmatter: {
    readonly name: string;
    readonly description: string;
    readonly author: string;
    readonly version: string;
    readonly official: boolean;
    readonly readonly: boolean;
  };
}

const { state: entries } = asyncRef([], load);

async function load() {
  const response = await fetch('registry.json');
  const data: Entry[] = await response.json();
  return data;
}

function boolString(bool: boolean) {
  return bool ? 'Yes' : 'No';
}
</script>

<template>
  <table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Author</th>
        <th>Version</th>
        <th>Official</th>
        <th>Read only</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="entry of entries" :key="entry.id">
        <td>
          <a :href="entry.script">{{ entry.frontmatter.name }}</a>
        </td>
        <td>
          <span>{{ entry.frontmatter.author }}</span>
        </td>
        <td>
          <span>{{ entry.frontmatter.version }}</span>
        </td>
        <td>
          <span>{{ boolString(entry.frontmatter.official) }}</span>
        </td>
        <td>
          <span>{{ boolString(entry.frontmatter.readonly) }}</span>
        </td>
      </tr>
    </tbody>
  </table>
</template>
