// dprint-ignore-file

// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as ffi from "node:ffi";

export const definitions = {
callofnil_client_version: { arguments: [], return: ffi.types.UINT_64 },
callofnil_ffi_version: { arguments: [], return: ffi.types.UINT_64 },
callofnil_free_str: { arguments: [ffi.types.POINTER], return: ffi.types.VOID },
callofnil_poll: { arguments: [ffi.types.POINTER], return: ffi.types.INT_32 },
callofnil_server_version: { arguments: [], return: ffi.types.UINT_64 },
callofnil_set_user_agent: { arguments: [ffi.types.POINTER], return: ffi.types.UINT_64 },
callofnil_user_agent: { arguments: [], return: ffi.types.UINT_64 },
callofnil_world: { arguments: [], return: ffi.types.UINT_64 },
} as const;
