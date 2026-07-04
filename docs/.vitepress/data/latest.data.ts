// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { delay } from "es-toolkit/promise";

export type Platform = "linux-x86_64-deb" | "linux-x86_64-rpm" | "windows-x86_64-nsis";

export interface Latest {
  readonly version: string;
  readonly pub_date: string;
  readonly platforms: Record<Platform, { url: string; }>;
}

interface Release {
  readonly browser_download_url: string;
}

export default {
  async load() {
    const latest: Latest = await getJson(
      "https://github.com/tsukilabs/nil/releases/latest/download/latest.json",
    );

    for (const [platform, { url }] of Object.entries(latest.platforms)) {
      await delay(100);
      const release: Release = await getJson(url);
      latest.platforms[platform as Platform] = {
        url: release.browser_download_url,
      };
    }

    return latest;
  },
};

async function getJson(url: string) {
  return fetch(url).then((response) => response.json());
}
