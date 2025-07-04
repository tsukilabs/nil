// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type ChatMessage = ChatMessageChatMessagePlayer;

interface ChatMessageChatMessagePlayer {
  readonly kind: 'player';
  readonly message: ChatMessagePlayer;
}

interface ChatMessagePlayer {
  readonly id: ChatMessageId;
  readonly author: PlayerId;
  readonly content: ChatMessageContent;
  readonly time: string;
}

type ChatMessageId = string;
type ChatMessageContent = string;
