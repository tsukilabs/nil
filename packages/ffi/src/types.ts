//! Copyright (C) Call of Nil contributors
//! SPDX-License-Identifier: AGPL-3.0-only

import type { definitions } from "./def";
import type { Option } from "@tb-dev/utils";
import type { Event } from "@tsukilabs/nil-bindings";
import type { DynamicLibraryResult } from "node:ffi";

export type Handle = DynamicLibraryResult<typeof definitions>;

export type ErrorHandler = (error: unknown) => void;
export type EventHandler = (event: Event) => void;

export interface QueueOptions {
  onError?: Option<ErrorHandler>;
  onEvent?: Option<EventHandler>;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface NilOptions extends QueueOptions {}

export interface DownloadLatestOptions {
  outDir?: Option<string>;
  overwrite?: boolean;
}

export type InitLatestOptions = DownloadLatestOptions & NilOptions;
