import { Nil } from "./packages/ffi/dist/index.js";

{
  using nil = new Nil("./target/release-ffi/nil_ffi");

  console.log(await nil.getClientVersion());
  console.log(await nil.getFfiVersion());
  console.log(await nil.getUserAgent());

  await nil.setUserAgent("nil-ffi");

  console.log(await nil.getUserAgent());
  console.log(await nil.getWorld() ?? "No active world");
}
