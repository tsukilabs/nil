// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { z } from 'zod';
import { isSafePathSegment } from './refine';

export const worldOptions = z.object({
  name: z.string().trim().min(3).max(30).refine(isSafePathSegment),
  size: z.int().min(100).max(200),
  allowCheats: z.boolean(),
  botDensity: z.number().min(0).max(3),
  botAdvancedStartRatio: z.number().min(0).max(1),
});
