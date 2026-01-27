// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetRemoteWorldResponse {
  readonly config: WorldConfig;
  readonly createdBy: PlayerId;
  readonly hasPassword: boolean;
  readonly activePlayers: number;
  readonly currentRound: RoundId;
}
