// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { getBotCoords } from './npc/bot';
import { getPlayerCoords } from './player';
import { getPrecursorCoords } from './npc/precursor';

export async function getRulerCoords(ruler: Ruler) {
  switch (ruler.kind) {
    case 'bot': {
      return getBotCoords(ruler.id);
    }
    case 'player': {
      return getPlayerCoords(ruler.id);
    }
    case 'precursor': {
      return getPrecursorCoords(ruler.id);
    }
  }
}
