// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Chat {
  readonly history: ChatHistory;
}

interface ChatHistory {
  readonly queue: ChatMessage[];
  readonly size: number;
}

interface ChatMessage {
  readonly id: ChatMessageId;
  readonly author: ChatMessageAuthor;
  readonly content: ChatMessageContent;
  readonly timestamp: string;
}

type ChatMessageAuthor = ChatMessageAuthorPlayer;

interface ChatMessageAuthorPlayer {
  readonly kind: 'player';
  readonly id: PlayerId;
}

type ChatMessageId = string;
type ChatMessageContent = string;
