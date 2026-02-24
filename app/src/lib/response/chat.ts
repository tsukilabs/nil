// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface GetChatHistoryResponse {
  readonly public: ChatHistory;
  readonly private: ChatHistory;
}
