// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface ScriptOutput {
  readonly stdout: readonly StdioMessage[];
  readonly stderr: readonly StdioMessage[];
}

interface StdioMessage {
  readonly id: number;
  readonly content: string;
  readonly time: string;
}
