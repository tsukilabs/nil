// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type ScriptId = number;

interface Script {
  readonly id: ScriptId;
  readonly owner: PlayerId;
  name: string;
  code: string;
}

interface Stdio {
  readonly stdout: readonly string[];
}
