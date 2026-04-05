// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { PlayerId } from '@/types/core/player';

export interface ChatHistory {
  readonly queue: ChatMessage[];
  readonly size: number;
}

export interface ChatMessage {
  readonly id: ChatMessageId;
  readonly kind: ChatMessageKind;
  readonly author: ChatMessageAuthor;
  readonly content: ChatMessageContent;
  readonly time: string;
}

export type ChatMessageKind = 'default';

export type ChatMessageAuthor = ChatMessageAuthorPlayer | ChatMessageAuthorSystem;

export interface ChatMessageAuthorSystem {
  readonly kind: 'system';
}

export interface ChatMessageAuthorPlayer {
  readonly kind: 'player';
  readonly id: PlayerId;
}

export type ChatMessageId = string;
export type ChatMessageContent = string;
