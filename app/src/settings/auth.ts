// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from "@/commands";
import type { Option } from "@tb-dev/utils";
import type { app_AuthSettings, ClientOptions } from "@tsukilabs/nil-bindings";

export class AuthSettingsImpl implements app_AuthSettings {
  public token: Option<string> = null;

  public async isTokenValid() {
    return Boolean(await commands.validateToken(this.token));
  }

  public async updateClient(options: Partial<ClientOptions> = {}) {
    await commands.updateClient({
      ...options,
      server: { kind: "remote" },
      authorizationToken: this.token,
    });
  }
}
