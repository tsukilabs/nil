// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type Ruler = RulerBot | RulerPlayer | RulerPrecursor;

type RulerId = Ruler['id'];
type RulerKind = Ruler['kind'];

interface RulerBot {
  readonly kind: 'bot';
  readonly id: BotId;
}

interface RulerPlayer {
  readonly kind: 'player';
  readonly id: PlayerId;
}

interface RulerPrecursor {
  readonly kind: 'precursor';
  readonly id: PrecursorId;
}
