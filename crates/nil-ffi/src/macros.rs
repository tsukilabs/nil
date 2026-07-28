// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[doc(hidden)]
#[macro_export]
macro_rules! push_ok {
  ($request_id:expr, $value:expr) => {{
    $crate::queue::push_ok($request_id, $value);
  }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! async_push_ok {
  ($request_id:expr, $value:expr) => {{
    $crate::RUNTIME.spawn(async move {
      $crate::queue::push_ok($request_id, $value);
    });
  }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! send {
  ($request_id:expr, $endpoint:ident) => {{
    use ::tap::Conv;
    $crate::RUNTIME.spawn(async move {
      let result = $crate::client::CLIENT
        .read()
        .await
        .$endpoint()
        .await
        .conv::<$crate::response::Result<_>>();

      $crate::queue::push_result($request_id, result);
    });
  }};
  ($request_id:expr, $endpoint:ident, $req_ptr:expr) => {{
    use ::tap::Conv;
    if $req_ptr.is_null() {
      $crate::queue::push_err($request_id, Status::ERR_NULL_POINTER);
    } else {
      match unsafe { $crate::json::deserialize_ptr($req_ptr) } {
        Ok(req) => {
          $crate::RUNTIME.spawn(async move {
            let result = $crate::client::CLIENT
              .read()
              .await
              .$endpoint(req)
              .await
              .conv::<$crate::response::Result<_>>();

            $crate::queue::push_result($request_id, result);
          });
        }
        Err(error) => {
          $crate::queue::push_err($request_id, error);
        }
      }
    }
  }};
}
