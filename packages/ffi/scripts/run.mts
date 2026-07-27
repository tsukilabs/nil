// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { join } from "node:path";
import { platform } from "node:os";
import { cwd } from "node:process";
import { Nil } from "../dist/index.js";

{
  const file = platform() === "win32" ? "nil_ffi" : "libnil_ffi";
  using nil = new Nil(join(cwd(), "target/release-ffi", file));

  console.log(await nil.getClientVersion());
  console.log(await nil.getFfiVersion());
  console.log(await nil.getUserAgent());

  await nil.setUserAgent("nil-ffi");

  console.log(await nil.getUserAgent());
  console.log(await nil.getWorld() ?? "No active world");
}
