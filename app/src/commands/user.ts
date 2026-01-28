// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { isRemote } from './index';
import { invoke } from '@tauri-apps/api/core';
import type { CreateUserRequest } from '@/lib/request';

export async function createUser(player: PlayerId, password: string) {
  if (await isRemote()) {
    const req: CreateUserRequest = { player, password };
    await invoke('create_user', { req });
  }
  else {
    throw new Error('Client is not connected to the remote server');
  }
}
