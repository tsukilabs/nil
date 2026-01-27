// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetBotCoordsRequest {
  readonly world: WorldId;
  readonly id: BotId;
}

export interface GetPublicBotRequest {
  readonly world: WorldId;
  readonly id: BotId;
}
