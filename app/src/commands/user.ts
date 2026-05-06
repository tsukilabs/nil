// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { invoke } from '@tauri-apps/api/core';
import type {
  CreateUserRequest,
  PlayerId,
  UserExistsRequest,
  UserExistsResponse,
} from '@/types/bindings';

export async function createUser(player: PlayerId, password: string) {
  const req: CreateUserRequest = { player, password };
  await invoke('create_user', { req });
}

export async function userExists(user: PlayerId) {
  const req: UserExistsRequest = { user };
  return invoke<UserExistsResponse>('user_exists', { req });
}
