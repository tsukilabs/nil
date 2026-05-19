// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from "@tb-dev/utils";
import { invoke } from "@tauri-apps/api/core";
import type { LocalServer, ServerAddr } from "@/types/server";
import type {
  AuthorizeRequest,
  AuthorizeResponse,
  GetServerKindResponse,
  PlayerId,
  ValidateTokenRequest,
  ValidateTokenResponse,
  VersionResponse,
  WorldOptions,
} from "@tsukilabs/nil-bindings";

export async function authorize(player: PlayerId, password: Option<string>) {
  const req: AuthorizeRequest = { player, password };
  return invoke<AuthorizeResponse>("authorize", { req });
}

export async function getLocalServerWorldId() {
  const serverKind = await getServerKind();
  return serverKind.kind === "local" ? serverKind.id : null;
}

export async function getServerAddr() {
  return invoke<ServerAddr>("get_server_addr");
}

export async function getServerKind() {
  return invoke<GetServerKindResponse>("get_server_kind");
}

export async function isServerReady() {
  return invoke<boolean>("is_server_ready");
}

export async function startServerWithOptions(worldOptions: WorldOptions) {
  return invoke<LocalServer>("start_server_with_options", { worldOptions });
}

export async function startServerWithSavedata(savedata: string) {
  return invoke<LocalServer>("start_server_with_savedata", { savedata });
}

export async function stopServer() {
  await invoke("stop_server");
}

export async function validateToken(token: Option<string>) {
  if (token) {
    const req: ValidateTokenRequest = { token };
    return invoke<ValidateTokenResponse>("validate_token", { req });
  }
  else {
    return null;
  }
}

export async function version() {
  return invoke<VersionResponse>("version");
}
