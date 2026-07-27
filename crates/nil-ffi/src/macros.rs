// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[doc(hidden)]
#[macro_export]
macro_rules! push_ok {
  ($value:expr) => {{
    let id = $crate::request::next_request_id();
    $crate::queue::push_ok(id, $value);
    id
  }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! async_push_ok {
  ($value:expr) => {{
    let id = $crate::request::next_request_id();
    $crate::RUNTIME.spawn(async move {
      $crate::queue::push_ok(id, $value);
    });

    id
  }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! send {
  ($endpoint:ident) => {{
    use ::tap::Conv;

    let id = $crate::request::next_request_id();
    $crate::RUNTIME.spawn(async move {
      let result = $crate::CLIENT
        .read()
        .await
        .$endpoint()
        .await
        .conv::<$crate::response::Result<_>>();

      $crate::queue::push_result(id, result);
    });

    id
  }};
  ($endpoint:ident, $req_ptr:expr) => {{
    use ::tap::Conv;

    let id = $crate::request::next_request_id();
    if $req_ptr.is_null() {
      $crate::queue::push_err(id, Status::ERR_NULL_POINTER);
    } else {
      let req = unsafe { ::std::ffi::CStr::from_ptr($req_ptr) };
      match req.to_str() {
        Ok(req) => {
          match ::serde_json::from_str(req) {
            Ok(req) => {
              $crate::RUNTIME.spawn(async move {
                let result = $crate::CLIENT
                  .read()
                  .await
                  .$endpoint(req)
                  .await
                  .conv::<$crate::response::Result<_>>();

                $crate::queue::push_result(id, result);
              });
            }
            Err(error) => $crate::queue::push_err(id, error),
          }
        }
        Err(error) => $crate::queue::push_err(id, error),
      }
    }

    id
  }};
}
