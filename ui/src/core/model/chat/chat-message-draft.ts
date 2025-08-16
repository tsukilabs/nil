// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { executeScriptChunk, pushChatMessage } from '@/commands';

const enum ChatMessageDraftKind {
  Default = 0,
  Lua = 1,
}

const regex = {
  lua: /^\$lua\s(.+)/,
} as const;

export class ChatMessageDraft {
  public text: Option<string> = null;

  public async send() {
    const [text, kind] = this.split();
    if (text) {
      switch (kind) {
        case ChatMessageDraftKind.Default: {
          await pushChatMessage(text);
          break;
        }
        case ChatMessageDraftKind.Lua: {
          await executeScriptChunk(text);
          break;
        }
      }
    }

    this.text = null;
  }

  private split() {
    let text = this.text;
    let kind = ChatMessageDraftKind.Default;

    if (text) {
      const result = regex.lua.exec(text);
      if (result) {
        text = result[1];
        kind = ChatMessageDraftKind.Lua;
      }
    }

    return [text, kind] as const;
  }
}
