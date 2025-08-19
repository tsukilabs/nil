import { extname, join, resolve } from 'node:path';
import { open, readdir, stat, writeFile } from 'node:fs/promises';

const collator = new Intl.Collator('en-US', {
  numeric: true,
  sensitivity: 'variant',
  usage: 'sort',
});

await getFiles().then((files) => Promise.all(files.map(sort)));

async function getFiles() {
  const files: string[] = [];
  const dir = resolve(import.meta.dirname, '../data');
  const entries = await readdir(dir, { encoding: 'utf8' });

  await Promise.all(
    entries.map(async (entry) => {
      entry = join(dir, entry);
      const stats = await stat(entry);
      if (extname(entry) === '.csv' && stats.isFile()) {
        files.push(entry);
      }
    }),
  );

  return files;
}

async function sort(file: string) {
  const handle = await open(file);
  const set = new Set<string>();

  try {
    const stream = handle.readLines({ encoding: 'utf8' });
    for await (const line of stream) {
      set.add(line.trim());
    }
  }
  finally {
    await handle.close();
  }

  const contents = set
    .values()
    .toArray()
    .toSorted((a, b) => collator.compare(a, b))
    .join('\n')
    .trim();

  await writeFile(file, contents, { encoding: 'utf8' });
}
