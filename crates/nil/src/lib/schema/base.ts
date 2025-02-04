import { z } from 'zod';

export const ip = z.string().ip({ version: 'v4' });
