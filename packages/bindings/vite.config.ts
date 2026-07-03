import dts from "unplugin-dts/vite";
import { defineConfig } from "vite";
import { extname, join } from "node:path";
import { writeFile } from "node:fs/promises";
import { Semaphore } from "es-toolkit/promise";

const semaphore = new Semaphore(50);

export default defineConfig({
  plugins: [
    dts({
      bundleTypes: false,
      afterBuild: async (files) => {
        const promises: Promise<void>[] = [];
        for (const [path, content] of files) {
          if (extname(path) === ".ts") {
            promises.push(prependLicense(path, content));
          }
        }

        await Promise.all(promises);
      },
    }),
  ],
  build: {
    outDir: "dist",
    emptyOutDir: true,
    minify: false,
    lib: {
      entry: join(import.meta.dirname, "src/index.ts"),
      formats: ["es"],
      fileName: "index",
    },
  },
});

async function prependLicense(path: string, content: string) {
  await semaphore.acquire();
  try {
    if (!content.includes("Copyright")) {
      let newContent = "//! Copyright (C) Call of Nil contributors\n";
      newContent += "//! SPDX-License-Identifier: AGPL-3.0-only\n\n";
      newContent += content;

      await writeFile(path, newContent, { encoding: "utf-8" });
    }
  }
  catch (err) {
    console.error(err);
  }
  finally {
    semaphore.release();
  }
}
