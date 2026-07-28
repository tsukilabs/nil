// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class RequestId {
  #current = 0;

  public next() {
    // See: https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX
    if (this.#current >= 4294967295) {
      this.#current = 0;
    }

    return ++this.#current;
  }
}
