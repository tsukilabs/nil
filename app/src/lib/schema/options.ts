// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { z } from 'zod';
import { isSafePathSegment } from './refine';

export const world = z.object({
  name: z.string().trim().min(3).max(30).refine(isSafePathSegment),
  size: z.int().min(100).max(200),
  allowCheats: z.boolean(),
});

export const player = z.object({
  id: z.string().trim().min(3).max(20).refine(isSafePathSegment),
});
