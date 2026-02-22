// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { toHtml } from '@/lib/highlighter';
import { remove } from '@tauri-apps/plugin-fs';

export class ScriptImpl implements Script {
  public readonly name: string;
  public readonly chunk: string;
  public readonly path: string;

  private constructor(script: Script) {
    this.name = script.name;
    this.chunk = script.chunk;
    this.path = script.path;
  }

  public async execute() {
    return commands.executeScript(this.chunk);
  }

  public async remove() {
    await remove(this.path);
  }

  public toHtml() {
    return toHtml(this.chunk);
  }

  public static create(script: Script) {
    if (script instanceof ScriptImpl) {
      return script;
    }

    return new ScriptImpl(script);
  }
}
