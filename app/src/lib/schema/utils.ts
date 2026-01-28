// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { z } from 'zod';

export const password = z.string().trim().min(3).max(50);
