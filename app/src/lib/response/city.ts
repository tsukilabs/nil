// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetPublicCityResponse {
  readonly city: PublicCity;
  readonly score: Option<number>;
}
