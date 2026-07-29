// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from "@tauri-apps/api/core";
import type { ClientOptions } from "@tsukilabs/nil-bindings";

export async function getClientVersion() {
  return invoke<string>("get_client_version");
}

export async function stopClient() {
  await invoke("stop_client");
}

export async function updateClient(options: ClientOptions) {
  if (options.server.kind !== "remote") {
    options.worldPassword = null;
    options.playerPassword = null;
    options.authorizationToken = null;
  }

  await invoke("update_client", { options });
}
