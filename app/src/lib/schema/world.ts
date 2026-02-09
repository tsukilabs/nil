// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { z } from 'zod';
import { isSafePathSegment } from './refine';

export const worldName = z.string()
  .trim()
  .min(3)
  .max(30)
  .refine(isSafePathSegment);

export const worldSize = z.int()
  .min(__CONSTS__.continentSizeMin)
  .max(__CONSTS__.continentSizeMax);

export const botDensity = z.number()
  .min(__CONSTS__.botDensityMin)
  .max(__CONSTS__.botDensityMax);

export const botAdvancedStartRatio = z.number()
  .min(__CONSTS__.botAdvancedStartRatioMin)
  .max(__CONSTS__.botAdvancedStartRatioMax);

export const worldOptions = z.object({
  name: worldName,
  size: worldSize,
  allowCheats: z.boolean(),
  botDensity,
  botAdvancedStartRatio,
});
