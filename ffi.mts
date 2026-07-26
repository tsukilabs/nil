import { Nil } from "./packages/ffi/dist/index.js";

{
  using nil = new Nil("./target/release-ffi/nil_ffi");

  console.log(nil.getClientVersion());
  console.log(nil.getFfiVersion());
  console.log(nil.getUserAgent());

  nil.setUserAgent("nil-ffi");

  console.log(nil.getUserAgent());
  console.log(nil.getWorld() ?? "No active world");
}
