import * as ffi from "node:ffi";
import { allocBuffer, definitions } from "./packages/ffi/src";

{
  using handle = ffi.dlopen(`./target/release-ffi/nil_ffi.${ffi.suffix}`, definitions);

  const versionPtr = handle.functions.callofnil_ffi_version();
  if (versionPtr) console.log(readStringPtr(versionPtr));

  const userAgentPtr = handle.functions.callofnil_user_agent();
  if (userAgentPtr) console.log(readStringPtr(userAgentPtr));

  const buffer = allocBuffer();
  const status = handle.functions.callofnil_set_user_agent("nil-ffi", buffer);
  if (status !== 0) {
    console.error(readStringPtr(buffer.readBigInt64LE()));
  }
  else {
    handle.functions.callofnil_free_str(buffer.readBigInt64LE());
  }

  const newUserAgentPtr = handle.functions.callofnil_user_agent();
  if (newUserAgentPtr) console.log(readStringPtr(newUserAgentPtr));

  const worldPtr = handle.functions.callofnil_world();
  if (worldPtr) {
    console.log(readStringPtr(worldPtr));
  }
  else {
    console.log("No world is currently set");
  }

  function readStringPtr(strPtr: bigint) {
    try {
      return ffi.toString(strPtr);
    }
    finally {
      handle.functions.callofnil_free_str(strPtr);
    }
  }
}
