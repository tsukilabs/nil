// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { z } from 'zod';

export const world = z.object({
  name: z.string().trim().min(3).max(30),
  size: z.number().int().positive().safe().min(10).max(255),
  allowCheats: z.boolean(),
});

export const player = z.object({
  id: z.string().trim().min(3).max(20),
});
