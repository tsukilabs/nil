// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(nonpoison_mutex, sync_nonpoison)]
#![expect(clippy::missing_safety_doc)]

mod queue;
mod request;
mod response;
mod status;

use crate::request::next_request_id;
use nil_client::Client;
use serde_json::to_string as serialize;
use std::ffi::{CStr, CString, c_char};
use std::ptr;
use std::sync::LazyLock;
use tap::Conv;
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};
use tokio::sync::RwLock;

pub use request::RequestId;
pub use response::{FfiResponse, FfiResult};
pub use status::Status;

static CLIENT: LazyLock<RwLock<Client>> = LazyLock::new(RwLock::default);

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
  RuntimeBuilder::new_multi_thread()
    .enable_all()
    .thread_name("callofnil-tokio")
    .build()
    .expect("failed to initialize tokio runtime")
});

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_client_version() -> RequestId {
  let id = next_request_id();
  queue::push_ok(id, nil_client::VERSION);
  id
}

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_ffi_version() -> RequestId {
  let id = next_request_id();
  queue::push_ok(id, env!("CARGO_PKG_VERSION"));
  id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn callofnil_free_str(ptr: *mut c_char) {
  if !ptr.is_null() {
    drop(unsafe { CString::from_raw(ptr) });
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_poll(out: *mut *mut c_char) -> Status {
  if out.is_null() {
    return Status::ERR_NULL_POINTER;
  }

  unsafe { *out = ptr::null_mut() };

  match queue::poll() {
    Ok(entry) => {
      match serialize(&entry) {
        Ok(json) => {
          if let Ok(json) = CString::new(json) {
            unsafe { *out = json.into_raw() };
            Status::OK
          } else {
            Status::ERR_INVALID_UTF8
          }
        }
        Err(_) => Status::ERR_SERIALIZATION,
      }
    }
    Err(status) => status,
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_server_version() -> RequestId {
  let id = next_request_id();
  RUNTIME.spawn(async move {
    let result = CLIENT
      .read()
      .await
      .version()
      .await
      .conv::<FfiResult<_>>();

    queue::push_result(id, result);
  });

  id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn callofnil_set_user_agent(user_agent: *const c_char) -> RequestId {
  let id = next_request_id();
  if user_agent.is_null() {
    queue::push_err(id, Status::ERR_NULL_POINTER);
  } else {
    let user_agent = unsafe { CStr::from_ptr(user_agent) };
    match user_agent.to_str() {
      Ok(user_agent) => {
        CLIENT
          .blocking_write()
          .set_user_agent(user_agent);

        queue::push_ok(id, ());
      }
      Err(err) => {
        queue::push_err(id, err);
      }
    }
  }

  id
}

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_user_agent() -> RequestId {
  let id = next_request_id();
  let user_agent = CLIENT
    .blocking_read()
    .user_agent()
    .to_owned();

  queue::push_ok(id, user_agent);
  id
}

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_world() -> RequestId {
  let id = next_request_id();
  match CLIENT.blocking_read().world() {
    Some(world) => queue::push_ok(id, world),
    None => queue::push_ok(id, None::<&str>),
  }

  id
}
