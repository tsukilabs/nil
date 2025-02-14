import { z } from 'zod';

export const world = z.object({
  size: z.number().int().positive().safe().min(10).max(255),
});

export const player = z.object({
  id: z.string().trim().min(3).max(20),
});
