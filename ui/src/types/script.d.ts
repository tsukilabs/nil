// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Script {
  readonly id: ScriptId;
  readonly owner: PlayerId;
  name: string;
  code: string;
}

type ScriptId = string;

type Stdout = readonly string[];

interface AddScriptRequest {
  name?: Option<string>;
  code?: Option<string>;
  owner: PlayerId;
}
