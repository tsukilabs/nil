// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PublicCity } from '@/types/core/city';

export interface GetPublicCityResponse {
  readonly city: PublicCity;
  readonly score: Option<number>;
}
