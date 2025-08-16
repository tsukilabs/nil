// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { leaveGame } from '@/core/game';
import { saveGame } from '@/core/savedata';
import { isPlayerTurn } from '@/composables/player/usePlayerTurn';

const enum ChatMessageDraftKind {
  Default = 'default',
  Academy = 'academy',
  EndTurn = 'end-turn',
  Farm = 'farm',
  IronMine = 'iron-mine',
  LeaveGame = 'leave-game',
  Lua = 'lua',
  Map = 'map',
  Nsr = 'nsr',
  Prefecture = 'prefecture',
  Quarry = 'quarry',
  SaveGame = 'save-game',
  Sawmill = 'sawmill',
  Silo = 'silo',
  Stable = 'stable',
  Wall = 'wall',
  Warehouse = 'warehouse',
}

type RegexMap = Readonly<Omit<Record<ChatMessageDraftKind, RegExp>, 'default'>>;

const regex: RegexMap = {
  [ChatMessageDraftKind.Academy]: /^\$academy$/i,
  [ChatMessageDraftKind.EndTurn]: /^\$end$/i,
  [ChatMessageDraftKind.Farm]: /^\$farm$/i,
  [ChatMessageDraftKind.IronMine]: /^\$iron-mine$/i,
  [ChatMessageDraftKind.LeaveGame]: /^\$leave$/i,
  [ChatMessageDraftKind.Lua]: /^\$lua\s(.+)/i,
  [ChatMessageDraftKind.Map]: /^\$map$/i,
  [ChatMessageDraftKind.Nsr]: /^\$nsr$/i,
  [ChatMessageDraftKind.Prefecture]: /^\$prefecture$/i,
  [ChatMessageDraftKind.Quarry]: /^\$quarry$/i,
  [ChatMessageDraftKind.SaveGame]: /^\$save$/i,
  [ChatMessageDraftKind.Sawmill]: /^\$sawmill$/i,
  [ChatMessageDraftKind.Silo]: /^\$silo$/i,
  [ChatMessageDraftKind.Stable]: /^\$stable$/i,
  [ChatMessageDraftKind.Wall]: /^\$wall$/i,
  [ChatMessageDraftKind.Warehouse]: /^\$warehouse$/i,
};

export class ChatMessageDraft {
  public text: Option<string> = null;

  public async send() {
    const [text, kind] = this.split();
    switch (kind) {
      case ChatMessageDraftKind.Default: {
        await commands.pushChatMessage(text);
        break;
      }
      case ChatMessageDraftKind.Academy: {
        await go('academy');
        break;
      }
      case ChatMessageDraftKind.EndTurn: {
        if (isPlayerTurn()) {
          await commands.endTurn();
        }
        break;
      }
      case ChatMessageDraftKind.Farm: {
        await go('farm');
        break;
      }
      case ChatMessageDraftKind.IronMine: {
        await go('iron-mine');
        break;
      }
      case ChatMessageDraftKind.LeaveGame: {
        await leaveGame();
        break;
      }
      case ChatMessageDraftKind.Lua: {
        await commands.executeScriptChunk(text);
        break;
      }
      case ChatMessageDraftKind.Map: {
        await go('continent');
        break;
      }
      case ChatMessageDraftKind.Nsr: {
        await go('nsr');
        break;
      }
      case ChatMessageDraftKind.Prefecture: {
        await go('prefecture');
        break;
      }
      case ChatMessageDraftKind.Quarry: {
        await go('quarry');
        break;
      }
      case ChatMessageDraftKind.SaveGame: {
        if (await commands.isHost()) {
          await saveGame();
        }
        break;
      }
      case ChatMessageDraftKind.Sawmill: {
        await go('sawmill');
        break;
      }
      case ChatMessageDraftKind.Silo: {
        await go('silo');
        break;
      }
      case ChatMessageDraftKind.Stable: {
        await go('stable');
        break;
      }
      case ChatMessageDraftKind.Wall: {
        await go('wall');
        break;
      }
      case ChatMessageDraftKind.Warehouse: {
        await go('warehouse');
        break;
      }
    }

    this.text = null;
  }

  private split() {
    let text = this.text?.trim();
    let kind = ChatMessageDraftKind.Default;

    if (text) {
      let result: Option<RegExpExecArray> = null;
      for (const [key, value] of Object.entries(regex)) {
        result = value.exec(text);
        if (result) {
          kind = key as ChatMessageDraftKind;
          break;
        }
      }

      if (result) {
        text = result.at(1);
      }
    }

    return [text ?? '', kind] as const;
  }
}
