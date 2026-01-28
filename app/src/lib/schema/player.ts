// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { z } from 'zod';
import { password } from './utils';
import { isSafePathSegment } from './refine';

export const playerId = z.string().trim().min(1).max(20).refine(isSafePathSegment);

export const playerOptions = z.object({
  id: playerId,
  password: password.nullish(),
});
