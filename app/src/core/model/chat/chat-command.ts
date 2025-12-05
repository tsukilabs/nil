// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { go } from '@/router';
import * as commands from '@/commands';
import { leaveGame } from '@/core/game';
import { saveGame } from '@/core/savedata';
import { ResourcesImpl } from '../resources';
import { isPlayerTurn } from '@/composables/player/usePlayerTurn';

const enum ChatCommandKind {
  Default = 'default',
  About = 'about',
  Academy = 'academy',
  EndTurn = 'end-turn',
  Farm = 'farm',
  Food = 'food',
  Iron = 'iron',
  IronMine = 'iron-mine',
  LeaveGame = 'leave-game',
  Map = 'map',
  Max = 'max',
  Prefecture = 'prefecture',
  Quarry = 'quarry',
  Resources = 'resources',
  SaveGame = 'save-game',
  Sawmill = 'sawmill',
  Silo = 'silo',
  SpawnBot = 'spawn-bot',
  Stable = 'stable',
  Stone = 'stone',
  Wall = 'wall',
  Warehouse = 'warehouse',
  Wood = 'wood',
}

type RegexMap = Readonly<Omit<Record<ChatCommandKind, RegExp>, 'default'>>;

const regex: RegexMap = {
  [ChatCommandKind.About]: /^\$about$/i,
  [ChatCommandKind.Academy]: /^\$academy$/i,
  [ChatCommandKind.EndTurn]: /^\$end$/i,
  [ChatCommandKind.Farm]: /^\$farm$/i,
  [ChatCommandKind.Food]: /^\$food(?:\s(\d+))?/i,
  [ChatCommandKind.Iron]: /^\$iron(?:\s(\d+))?/i,
  [ChatCommandKind.IronMine]: /^\$iron-mine$/i,
  [ChatCommandKind.LeaveGame]: /^\$leave$/i,
  [ChatCommandKind.Map]: /^\$map$/i,
  [ChatCommandKind.Max]: /^\$max$/i,
  [ChatCommandKind.Prefecture]: /^\$prefecture$/i,
  [ChatCommandKind.Quarry]: /^\$quarry$/i,
  [ChatCommandKind.Resources]: /^\$res(?:\s(\d+))?/i,
  [ChatCommandKind.SaveGame]: /^\$save$/i,
  [ChatCommandKind.Sawmill]: /^\$sawmill$/i,
  [ChatCommandKind.Silo]: /^\$silo$/i,
  [ChatCommandKind.SpawnBot]: /^\$spawn(?:\s(.+))?/i,
  [ChatCommandKind.Stable]: /^\$stable$/i,
  [ChatCommandKind.Stone]: /^\$stone(?:\s(\d+))?/i,
  [ChatCommandKind.Wall]: /^\$wall$/i,
  [ChatCommandKind.Warehouse]: /^\$warehouse$/i,
  [ChatCommandKind.Wood]: /^\$wood(?:\s(\d+))?/i,
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
      case ChatCommandKind.About: {
        await go('about');
        break;
      }
      case ChatCommandKind.Academy: {
        await go('academy');
        break;
      }
      case ChatCommandKind.EndTurn: {
        if (isPlayerTurn()) {
          await commands.setPlayerReady(true);
        }
        break;
      }
      case ChatCommandKind.Farm: {
        await go('farm');
        break;
      }
      case ChatCommandKind.Food: {
        await commands.cheatSetFood(Number.parseInt(this.text));
        break;
      }
      case ChatCommandKind.Iron: {
        await commands.cheatSetIron(Number.parseInt(this.text));
        break;
      }
      case ChatCommandKind.IronMine: {
        await go('iron-mine');
        break;
      }
      case ChatCommandKind.LeaveGame: {
        await leaveGame();
        break;
      }
      case ChatCommandKind.Map: {
        await go('continent');
        break;
      }
      case ChatCommandKind.Max: {
        await setMax();
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
      case ChatCommandKind.Resources: {
        const amount = Number.parseInt(this.text);
        await commands.cheatSetResources(ResourcesImpl.splat(amount));
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
      case ChatCommandKind.SpawnBot: {
        await commands.cheatSpawnBot(this.text);
        break;
      }
      case ChatCommandKind.Stable: {
        await go('stable');
        break;
      }
      case ChatCommandKind.Stone: {
        await commands.cheatSetStone(Number.parseInt(this.text));
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
      case ChatCommandKind.Wood: {
        await commands.cheatSetWood(Number.parseInt(this.text));
        break;
      }
    }
  }
}

async function setMax() {
  const player = NIL.player.getPlayer()?.toRuler();
  const promises = [commands.cheatSetMaxResources()];
  for (const coord of NIL.player.getCoords()) {
    promises.push(
      commands.cheatSetMaxInfrastructure(coord),
      commands.cheatSpawnPersonnel(coord, 10_000, player),
    );
  }

  await Promise.all(promises);
}
