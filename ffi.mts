import * as ffi from "node:ffi";

{
  using handle = ffi.dlopen(`./target/release/callofnil.${ffi.suffix}`, {
    callofnil_free_str: { arguments: ["ptr"], return: "void" },
    callofnil_version: { arguments: [], return: "ptr" },
  });

  const ptr = handle.functions.callofnil_version();
  if (ptr) {
    try {
      console.log(ffi.toString(ptr));
    }
    finally {
      handle.functions.callofnil_free_str(ptr);
    }
  }
}
