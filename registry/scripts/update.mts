// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { join } from 'node:path';
import matter from 'gray-matter';
import { cwd } from 'node:process';
import { readdir, readFile, stat, writeFile } from 'node:fs/promises';

const NSR = 'https://nil.dev.br/nsr';

interface Entry {
  id: string;
  readme: string;
  script: string;
  frontmatter: Record<string, unknown>;
}

const index: Entry[] = [];

const registry = join(cwd(), 'public');
const entries = await readdir(registry, { encoding: 'utf8' });
await Promise.all(entries.map(checkEntry));

await write();

async function checkEntry(entry: string) {
  const path = join(registry, entry);
  const stats = await stat(path);
  if (stats.isDirectory()) {
    const readmePath = join(path, 'readme.md');
    const readme = await readFile(readmePath, { encoding: 'utf8' });
    index.push({
      id: entry,
      readme: `${NSR}/${entry}/readme.md`,
      script: `${NSR}/${entry}/script.lua`,
      frontmatter: matter(readme).data,
    });
  }
}

async function write() {
  const path = join(registry, 'registry.json');
  const data = JSON.stringify(index, null, 0);
  await writeFile(path, data, { encoding: 'utf8' });
}
