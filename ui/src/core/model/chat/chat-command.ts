// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { leaveGame } from '@/core/game';
import { saveGame } from '@/core/savedata';
import { isPlayerTurn } from '@/composables/player/usePlayerTurn';

const enum ChatCommandKind {
  Default = 'default',
  Academy = 'academy',
  EndTurn = 'end-turn',
  Farm = 'farm',
  IronMine = 'iron-mine',
  Kami = 'kami',
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

type RegexMap = Readonly<Omit<Record<ChatCommandKind, RegExp>, 'default'>>;

const regex: RegexMap = {
  [ChatCommandKind.Academy]: /^\$academy$/i,
  [ChatCommandKind.EndTurn]: /^\$end$/i,
  [ChatCommandKind.Farm]: /^\$farm$/i,
  [ChatCommandKind.IronMine]: /^\$iron-mine$/i,
  [ChatCommandKind.Kami]: /^\$kami$/i,
  [ChatCommandKind.LeaveGame]: /^\$leave$/i,
  [ChatCommandKind.Lua]: /^\$lua\s(.+)/i,
  [ChatCommandKind.Map]: /^\$map$/i,
  [ChatCommandKind.Nsr]: /^\$nsr$/i,
  [ChatCommandKind.Prefecture]: /^\$prefecture$/i,
  [ChatCommandKind.Quarry]: /^\$quarry$/i,
  [ChatCommandKind.SaveGame]: /^\$save$/i,
  [ChatCommandKind.Sawmill]: /^\$sawmill$/i,
  [ChatCommandKind.Silo]: /^\$silo$/i,
  [ChatCommandKind.Stable]: /^\$stable$/i,
  [ChatCommandKind.Wall]: /^\$wall$/i,
  [ChatCommandKind.Warehouse]: /^\$warehouse$/i,
};

export class ChatCommand {
  private readonly text: string;
  private readonly kind: ChatCommandKind;

  constructor(draft: Option<string>) {
    this.text = draft?.trim() ?? '';
    this.kind = ChatCommandKind.Default;

    if (this.text.length > 0) {
      let result: Option<RegExpExecArray> = null;
      for (const [key, value] of Object.entries(regex)) {
        result = value.exec(this.text);
        if (result) {
          this.kind = key as ChatCommandKind;
          break;
        }
      }

      if (result) {
        this.text = result.at(1) ?? '';
      }
    }
  }

  public async execute() {
    switch (this.kind) {
      case ChatCommandKind.Default: {
        await commands.pushChatMessage(this.text);
        break;
      }
      case ChatCommandKind.Academy: {
        await go('academy');
        break;
      }
      case ChatCommandKind.EndTurn: {
        if (isPlayerTurn()) {
          await commands.endTurn();
        }
        break;
      }
      case ChatCommandKind.Farm: {
        await go('farm');
        break;
      }
      case ChatCommandKind.IronMine: {
        await go('iron-mine');
        break;
      }
      case ChatCommandKind.Kami: {
        await setKamiMode();
        break;
      }
      case ChatCommandKind.LeaveGame: {
        await leaveGame();
        break;
      }
      case ChatCommandKind.Lua: {
        await commands.executeScriptChunk(this.text);
        break;
      }
      case ChatCommandKind.Map: {
        await go('continent');
        break;
      }
      case ChatCommandKind.Nsr: {
        await go('nsr');
        break;
      }
      case ChatCommandKind.Prefecture: {
        await go('prefecture');
        break;
      }
      case ChatCommandKind.Quarry: {
        await go('quarry');
        break;
      }
      case ChatCommandKind.SaveGame: {
        if (await commands.isHost()) {
          await saveGame();
        }
        break;
      }
      case ChatCommandKind.Sawmill: {
        await go('sawmill');
        break;
      }
      case ChatCommandKind.Silo: {
        await go('silo');
        break;
      }
      case ChatCommandKind.Stable: {
        await go('stable');
        break;
      }
      case ChatCommandKind.Wall: {
        await go('wall');
        break;
      }
      case ChatCommandKind.Warehouse: {
        await go('warehouse');
        break;
      }
    }
  }
}

async function setKamiMode() {
  await commands.cheatSetMaxResources();
  await Promise.all(NIL.player.coords().map(commands.cheatSetMaxInfrastructure));
}
