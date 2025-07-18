// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface NsrScript {
  readonly id: string;
  readonly readme: string;
  readonly script: string;
  readonly frontmatter: NsrScriptFrontmatter;
}

interface NsrScriptFrontmatter {
  readonly name: string;
  readonly description: string;
  readonly author: string;
  readonly version: string;
  readonly official: boolean;
  readonly readonly: boolean;
}
