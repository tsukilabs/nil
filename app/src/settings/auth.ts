// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';

export class AuthSettingsImpl implements AuthSettings {
  public token: Option<string> = null;

  public async isTokenValid() {
    return Boolean(await commands.validateToken(this.token));
  }

  public async updateClient(options: Partial<ClientOptions> = {}) {
    await commands.updateClient({
      ...options,
      serverAddr: { kind: 'remote' },
      authorizationToken: this.token,
    });
  }
}
