// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Script {
  readonly id: ScriptId;
  readonly owner: PlayerId;
  name: string;
  code: string;
}

type ScriptId = number;
