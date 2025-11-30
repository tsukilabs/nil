// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface PublicBot {
  readonly id: BotId;
}

interface Bot extends PublicBot {
  readonly ethics: Ethics;
  readonly resources: Resources;
}

type BotId = string;
