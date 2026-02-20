// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { StdioMessageImpl } from './stdio-message';

export class ScriptOutputImpl implements ScriptOutput {
  public readonly stdout: readonly StdioMessageImpl[];
  public readonly stderr: readonly StdioMessageImpl[];

  private constructor(output: ScriptOutput) {
    this.stdout = output.stdout.map((it) => StdioMessageImpl.create(it));
    this.stderr = output.stderr.map((it) => StdioMessageImpl.create(it));
  }

  public *[Symbol.iterator]() {
    yield* this.toArray();
  }

  public toArray() {
    return [...this.stdout, ...this.stderr];
  }

  public toSortedArray() {
    return this.toArray().sort((a, b) => a.compareDateAsc(b));
  }

  public static create(output: ScriptOutput) {
    if (output instanceof ScriptOutputImpl) {
      return output;
    }

    return new ScriptOutputImpl(output);
  }

  public static toArray(output: ScriptOutput) {
    return this.create(output).toArray();
  }

  public static toSortedArray(output: ScriptOutput) {
    return this.create(output).toSortedArray();
  }
}
