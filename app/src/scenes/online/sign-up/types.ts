// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export interface NewUser {
  name: Option<PlayerId>;
  password: Option<string>;
  password2: Option<string>;
}
